#![no_main]
#![no_std]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    println!("{}",_info);
    loop{}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core ::fmt::Write;
    println!("Testing println function with {}", 9.5);
    panic!("this is a panic message");
    loop {}
}
