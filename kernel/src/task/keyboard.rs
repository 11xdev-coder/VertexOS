use crate::{print, println, commands::handle_command, vga_buffer::WRITER};
use spin::Mutex;
use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use futures_util::{
    stream::{Stream, StreamExt},
    task::AtomicWaker,
};
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};


static WAKER: AtomicWaker = AtomicWaker::new();
static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
pub const INPUT_BUFFER_SIZE: usize = 256; // Maximum command length

lazy_static! {
    static ref INPUT_BUFFER: Mutex<[u8; INPUT_BUFFER_SIZE]> = Mutex::new([0; INPUT_BUFFER_SIZE]);
    static ref INPUT_BUFFER_POSITION: Mutex<usize> = Mutex::new(0);
}


pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        ScancodeStream { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        // get a reference to the initialized scancode queue
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("scancode queue not initialized");

        // fast path
        if let Ok(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode)); // ready if succeed
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Ok(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}

pub async fn print_keypress() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        if character == '\n' {
                            // Enter key pressed
                            println!(""); // Move to a new line
                        
                            // Get the command from the buffer
                            let mut buffer_lock = INPUT_BUFFER.lock();
                            let position = *INPUT_BUFFER_POSITION.lock();
                            let command = &buffer_lock[..position]; // Slice up to the filled position
                        
                            // Handle the command
                            handle_command(command);
                        
                            // Clear the buffer and reset the position
                            *buffer_lock = [0; INPUT_BUFFER_SIZE];
                            *INPUT_BUFFER_POSITION.lock() = 0;
                        
                            // Print the prompt for the next command
                            print!("> ");
                        } else if character == '\x08' {
                            // Backspace key pressed
                            let mut writer = WRITER.lock();
                            writer.remove_previous_symbol();
                            let mut position = INPUT_BUFFER_POSITION.lock();
                            if *position > 0 {
                                *position -= 1;
                            }
                        } else {
                            print!("{}", character);
                            let mut buffer = INPUT_BUFFER.lock();
                            let mut position = INPUT_BUFFER_POSITION.lock();
                            if *position < INPUT_BUFFER_SIZE {
                                buffer[*position] = character as u8;
                                *position += 1;
                            }
                        }
                    },
                    DecodedKey::RawKey(key) => print!("{:?}", key),
                }
            }
        }
    }
    
}