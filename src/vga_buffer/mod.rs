use spin::Mutex;
use lazy_static::lazy_static;
use volatile::Volatile;
use core::fmt;

mod Write_wrapper;
use Write_wrapper::Writer;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color{
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
//memory alignment is [u8], and could be casted as u8.
pub struct ColorCode(u8);

impl ColorCode{
    fn new(foreground: Color, background: Color)-> ColorCode{
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
//repr(C) means the struct will be memory aligned like C struct in the order of members.
//ascii_characters aligns [u8] -> which is 8 bits of memory, then 
//ColorCode alignes [u8] -> another 8 bits of memory
//EACH VGA Text Buffer is 16 bits/ 2 bytes long
struct ScreenChar{
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer{
    chars:[[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}


const VGA_BUFFER_POINTER: *mut Buffer = 0xb8000 as *mut Buffer;
lazy_static!{
    pub static ref WRITER:Mutex<Writer> = Mutex::new( Writer{ 
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black), 
        buffer: unsafe{&mut  *VGA_BUFFER_POINTER},
    });
}

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
pub fn _print(args: fmt::Arguments){
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

// pub fn print_something(){
//     use core::fmt::Write;
//     let mut writer = Writer{
//         column_position: 0,
//         color_code: ColorCode::new(Color::White, Color::Black),
//         buffer: unsafe{&mut  *VGA_BUFFER_POINTER},
//     };

//     writer.write_byte(b'H');
//     writer.write_string("ello ");
//     writer.write_string("World!");
//     write!(writer, "There {} , {} ", 170, 9.0/7.0).unwrap();
// }
