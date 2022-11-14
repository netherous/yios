#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(yios::test_runner)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;
use yios::*;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    println!("{}",_info);
    loop{}
}
#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    yios::test_panic_handler(_info);
    hlt_loop();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("TEMOC{}", "! Loves the world, and Lets go !");

    yios::init();

    // x86_64::instructions::interrupts::int3();
    //
    // unsafe{
    //     *(0xdeadbeef as *mut usize) = 42;
    // };
    
    fn stack_overflow () {stack_overflow();}

    // stack_overflow();

    #[cfg(test)]  
    test_main();

    println!("Testing println function with {}", 9.5);
    // panic!("this is a panic message");
    hlt_loop();
}

