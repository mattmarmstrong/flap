#[allow(dead_code, unused)]
use core::{fmt, str};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Color {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(background: Color, foreground: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    char: u8,
    color: ColorCode,
}

impl ScreenChar {
    fn new(char: u8, color: ColorCode) -> Self {
        ScreenChar { char, color }
    }
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    x_pos: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_bytes(s);
        Ok(())
    }
}

impl Writer {
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.x_pos >= BUFFER_WIDTH {
                    self.new_line();
                }
                let y_pos: usize = BUFFER_HEIGHT - 1;
                let color = self.color_code;
                self.buffer.chars[y_pos][self.x_pos].write(ScreenChar::new(byte, color));
                self.x_pos += 1;
            }
        }
    }

    fn write_bytes(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for y in 1..BUFFER_HEIGHT {
            for x in 1..BUFFER_WIDTH {
                let char = self.buffer.chars[y][x].read();
                self.buffer.chars[y - 1][x].write(char);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.x_pos = 1;
    }

    fn clear_row(&mut self, y: usize) {
        let whitespace = ScreenChar::new(b' ', self.color_code);
        for x in 0..BUFFER_WIDTH {
            self.buffer.chars[y][x].write(whitespace);
        }
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        x_pos: 1_usize,
        color_code: ColorCode::new(Color::Black, Color::Cyan,),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
