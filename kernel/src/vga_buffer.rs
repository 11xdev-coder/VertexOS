use volatile::Volatile;
use lazy_static::lazy_static;
use core::{fmt, panic::PanicInfo};
use spin::Mutex;
use x86_64::instructions::port::Port;
use alloc::format;

#[allow(dead_code)] // disabling warnings when compiler sees unused code
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // enabling copy semantics
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

const PROMPT_LENGTH: usize = 2; // "> " symbol at start is 2 symbols
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode {
    foreground: Color,
    background: Color,
}

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode { foreground, background }
    }

    pub fn get_foreground(&self) -> Color {
        self.foreground
    }

    pub fn get_background(&self) -> Color {
        self.background
    }

    // You'll need this to convert the ColorCode to a u8 for the VGA buffer
    pub fn to_u8(&self) -> u8 {
        (self.background as u8) << 4 | (self.foreground as u8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: u8,
}

#[repr(transparent)] // ensure that Buffer has the same memory layout as its single field.
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

/// Like the `println!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Prints the given formatted string to the VGA text buffer
/// through the global `WRITER` instance.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[allow(dead_code)]
impl Writer {
    /// Writes an ASCII byte to the buffer.
    ///
    /// Wraps lines at `BUFFER_WIDTH`. Supports the `\n` newline character.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code.to_u8();
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }

        self.update_cursor();
    }

    /// Writes the given ASCII string to the buffer.
    ///
    /// Wraps lines at `BUFFER_WIDTH`. Supports the `\n` newline character. Does **not**
    /// support strings with non-ASCII characters, since they can't be printed in the VGA text
    /// mode.
    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    // Writes a byte at a specific position without altering the cursor or column position
    pub fn write_byte_at(&mut self, byte: u8, column: usize, row: usize) {
        let color_code = self.color_code.to_u8();
        self.buffer.chars[row][column].write(ScreenChar {
            ascii_character: byte,
            color_code,
        });
    }

    pub fn write_string_at(&mut self, s: &str, mut column: usize, mut row: usize) {
        for byte in s.bytes() {
            if byte == b'\n' {
                // If newline, move to the next line and reset column
                row += 1;
                column = 0;
                continue;
            }
    
            if column >= BUFFER_WIDTH {
                // If end of line, move to the next line
                row += 1;
                column = 0;
            }
    
            if row >= BUFFER_HEIGHT {
                // Scroll up the text to make room for more text
                self.new_line();
                row = BUFFER_HEIGHT - 1; // Set row to the last line after scrolling
            }
    
            // Write the byte and increment the column
            self.write_byte_at(byte, column, row);
            column += 1;
        }
    }        

    /// Shifts all lines one line up and clears the last row.
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;

        self.update_cursor();
    }

    /// Clears a row by overwriting it with blank characters.
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code.to_u8(),
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn set_column(&mut self, column: usize) {
        self.column_position = column;
    }

    pub fn get_column(&self) -> usize {
        self.column_position
    }

    pub fn remove_previous_symbol(&mut self) {
        if self.column_position > PROMPT_LENGTH {
            self.column_position -= 1; // Move back one position
            self.write_byte(b' '); // Overwrite the character with a space
            self.column_position -= 1; // Move back again to overwrite the same position next time
        } else if self.column_position == PROMPT_LENGTH {
            // Check if the last two characters are the prompt
            let row = BUFFER_HEIGHT - 1; // Assuming current row is always the last one
            let prev_char = self.buffer.chars[row][self.column_position - 1].read();
            let prev_prev_char = self.buffer.chars[row][self.column_position - 2].read();
            
            if !(prev_char.ascii_character == b' ' && prev_prev_char.ascii_character == b'>') {
                // If the last two characters are not the prompt, remove the last symbol
                self.column_position -= 1; // Move back one position
                self.write_byte(b' '); // Overwrite the character with a space
                self.column_position -= 1; // Move back again to overwrite the same position next time
            }
        } else if self.column_position <= 1 {
            // If we are at the start of a line (after a newline), move the text down
            self.move_text_down();
            self.column_position = BUFFER_WIDTH; // Set cursor to the end of the previous line
        }

        self.update_cursor();
    }           

    /// Moves all text down by one line, creating space at the top row.
    fn move_text_down(&mut self) {
        for row in (1..BUFFER_HEIGHT).rev() {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row - 1][col].read();
                self.buffer.chars[row][col].write(character);
            }
        }
    }

    fn update_cursor(&self) {
        let row = BUFFER_HEIGHT - 1; // The current row is always the last one
        let position = row * BUFFER_WIDTH + self.column_position;
    
        // VGA control registers
        let vga_index_port = 0x3D4;
        let vga_data_port = 0x3D5;
    
        let mut index_port = Port::new(vga_index_port);
        let mut data_port = Port::new(vga_data_port);
    
        unsafe {
            // Set the high cursor byte
            index_port.write(0x0E as u8);
            data_port.write((position >> 8) as u8);
            // Set the low cursor byte
            index_port.write(0x0F as u8);
            data_port.write(position as u8);
        }
    }   

    /// Sets the color code for the Writer.
    fn set_color(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }

    pub fn set_screen_color(&mut self, background_color: Color) {
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let color_code = ColorCode::new(self.color_code.get_foreground(), background_color).to_u8();
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: b' ',
                    color_code,
                });
            }
        }
        self.column_position = 0;
        self.update_cursor();
    }

    pub fn reset_color(&mut self) {
        self.set_color(ColorCode::new(Color::White, Color::Black));
    }

    // BSOD ------------------------------------------------------------------------------
    pub fn bsod_title(&mut self) {
        // Calculate the position to center the "VertexDOS panicked" message
        let message = "VertexDOS panicked";
        let row = BUFFER_HEIGHT / 3; // Slightly more to the top
        let column = (BUFFER_WIDTH - message.len()) / 2; // Centered
    
        // Set the color for the text
        self.set_color(ColorCode::new(Color::Blue, Color::White));
    
        // Write the message at the calculated position
        self.write_string_at(message, column, row);
    }    
    
    pub fn bsod_panic_message(&mut self, info: &PanicInfo) {
        let message = format!("{}", info);
        let row = BUFFER_HEIGHT / 2; // Start in the middle of the screen
        let column = (BUFFER_WIDTH - message.len()) / 3;

        self.set_color(ColorCode::new(Color::White, Color::Blue));

        self.write_string_at(&message, column, row);
    }
    // --------------------------------------------------------------------------------------------------
    
}


impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}