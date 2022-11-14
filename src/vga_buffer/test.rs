use crate::println;
use super::{WRITER,BUFFER_HEIGHT};
#[test_case]
fn test_println_amount(){
    for _ in 0..300{
        println!("test_println_amount output");
    }
}

#[test_case]
fn test_println_output(){
    use x86_64::instructions::interrupts;
    use core::fmt::Write;
    let s = "test_println_output string";
    interrupts::without_interrupts(||{
        let mut w = WRITER.lock();
        writeln!(w, "\n{}", s).expect("writeln failed");
        for (i,b) in s.chars().enumerate(){
            let here =w.buffer.chars[BUFFER_HEIGHT-2][i].read(); 
            assert_eq!(b, char::from(here.ascii_character));
        }
    });
}
