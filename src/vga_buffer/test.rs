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
    let s = "test_println_output string";
    println!("{}", s);
    for (i,b) in s.chars().enumerate(){
       let here = WRITER.lock().buffer.chars[BUFFER_HEIGHT-2][i].read(); 
       assert_eq!(b, char::from(here.ascii_character));
    }
}
