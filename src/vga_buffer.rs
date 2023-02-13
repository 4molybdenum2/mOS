#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]

#[repr(transparent)]
struct ColorCode(u8);


#[repr(C)]
struct ScreenChar {
    ascii_character : u8,
    color_code : ColorCode
}

const BUFFER_WIDTH : usize = 80
const BUFFER_HEIGHT : usize = 25


#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
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

    }
}