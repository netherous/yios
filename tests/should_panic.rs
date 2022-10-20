#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use yios::{QemuExitCode, exit_qemu, serial_print,serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn panic (_info: &PanicInfo)->!{

    serial_println!("[passed]");
    exit_qemu(QemuExitCode::Success);
    loop{}
}

pub fn test_runner(tests: &[&dyn Fn()]){
    serial_println!("Total {} tests", tests.len());
    for test in tests{
        test();
        serial_println!("[test did not panic]");
        exit_qemu(QemuExitCode::Fail);
    }
    exit_qemu(QemuExitCode::Success);
}


#[test_case]
fn should_fail(){
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0,1);
}
