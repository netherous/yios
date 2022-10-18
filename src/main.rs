#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;

mod vga_buffer;
mod serial;

#[derive(Clone, Copy,Debug,PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode{
    Success = 0x10,
    Fail = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode){
    use x86_64::instructions::port::Port;
    unsafe{
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    println!("{}",_info);
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo)->!{
    serial_println!("[fail]\n");
    serial_println!("Panic!: {}\n", _info);
    exit_qemu(QemuExitCode::Success);
    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

    #[cfg(test)]  
    test_main();
    println!("Testing println function with {}", 9.5);
    panic!("this is a panic message");

    loop {}
}

#[cfg(test)]
// &[] is a slice, which is a fat pointer to an continuous segment of memory, 
// containing (pointer: *type, length of the segment)
// dyn Fn() means object that implement trait Fn(), which is trait object
fn test_runner( tests: &[&dyn Testable]){
    serial_println!("Total {} tests", tests.len());
    for test in tests{
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn assertion_template(){
    assert_eq!(1,1);
}
#[test_case]
fn assertion_template2(){
    assert_ne!(0,1);
}

pub trait Testable{
    fn run(&self)->();
}

impl <T: Fn()> Testable for T{
    fn run(&self){
        let s = core::any::type_name::<T>();
        serial_print!("{}...\t", s);
        self();
        serial_println!("[passed]")
    }
}
