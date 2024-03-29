use core::fmt;
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

#[allow(dead_code)]


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character : u8,
    color_code : ColorCode
}

const BUFFER_WIDTH : usize = 80;
const BUFFER_HEIGHT : usize = 25;



#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

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


pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

// defines the color code (foreground and background) of the vga text buffer
impl ColorCode {
    fn new(foreground : Color, background : Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// implements writer to write to the vga buffer
impl Writer {

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
                b'\n' => self.new_line(),

                byte => {
                    if self.column_position >= BUFFER_WIDTH {
                        self.new_line();
                    }
                    // else write byte


                    let row = BUFFER_HEIGHT -1;
                    let col = self.column_position;
                    
                    let color_code = self.color_code;


                    self.buffer.chars[row][col].write(
                        ScreenChar{
                            ascii_character: byte, 
                            color_code: color_code
                        }
                    );
                    self.column_position += 1;
                },
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let ch = self.buffer.chars[row][col].read();
                self.buffer.chars[row-1][col].write(ch);
                
            }
        }

        self.clear_row(BUFFER_HEIGHT-1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row : usize) {
        let blank_ch = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };


        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank_ch);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {

                0x20..=0x7e  | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// lazy static because statics in Rust are generated at compile time, one time init of statiics of non-const functions
// is a problem in Rust

lazy_static! {

    // Spin locks to make our writer immutable // gloval static writer
    
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)},
    });
}



// These macros are copied from the standard println! macros

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}



#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;


    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    })


}




/// Tests
#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";

    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
    
}