use crate::{gdt, print, println};
use crate::vga_buffer::WRITER;
use crate::commands::handle_command;
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
use spin::Mutex;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)] // each variant is represented like u8
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub const INPUT_BUFFER_SIZE: usize = 256; // Maximum command length

lazy_static! {
    static ref INPUT_BUFFER: Mutex<[u8; INPUT_BUFFER_SIZE]> = Mutex::new([0; INPUT_BUFFER_SIZE]);
    static ref INPUT_BUFFER_POSITION: Mutex<usize> = Mutex::new(0);
}

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);

        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard_interrupt_handler);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

// double fault can occur when a second exception occurs when handling the first exception
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8()); // need to notify the end so system can process the next interrupt
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() }; // reading the scancode of the key-press

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

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8()); // process the next interrupt
    }
}