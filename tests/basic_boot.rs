#![test_runner(yios::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn panic (_info: &PanicInfo)->!{

    yios::test_panic_handler(_info);
    loop{}
}

use yios::println;
#[test_case]
fn test_println(){
    println!("test_println! output");
}
