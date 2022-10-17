use super::*;

pub struct Writer{
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8){
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT -1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write( ScreenChar{
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s:&str){
        for byte in s.bytes(){
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

}
impl Writer{
    fn new_line(&mut self){
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH{
                let cha = self.buffer.chars[row][col].read();
                self.buffer.chars[row-1][col].write(cha);
            }
        }
        self.column_position = 0;
        self.clear_row(BUFFER_HEIGHT-1);
    }
    fn clear_row(&mut self, row: usize){
        let space = ScreenChar{
            ascii_character: b' ',
            color_code: ColorCode::new(Color::Black, Color::Black), 
        }; 
        for col in 0..BUFFER_WIDTH{
            self.buffer.chars[row][col].write(space);    
        }
    }
}
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
