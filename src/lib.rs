#![feature(abi_x86_interrupt)]
#![cfg_attr(test, no_main)]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![no_std]


use core::panic::PanicInfo;

pub mod vga_buffer;
pub mod serial;
pub mod interrupts;
pub mod gdt;
pub mod pic8259;

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

#[panic_handler]
#[cfg(test)]
fn panic(_info: &PanicInfo)->!{
    test_panic_handler(_info);
    loop{}
}

pub fn test_panic_handler(_info: &PanicInfo)-> !{
    serial_println!("[fail]\n");
    serial_println!("Panic!: {}\n", _info);
    exit_qemu(QemuExitCode::Success);
    loop{}
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

// &[] is a slice, which is a fat pointer to an continuous segment of memory, 
// containing (pointer: *type, length of the segment)
// dyn Fn() means object that implement trait Fn(), which is trait object
pub fn test_runner( tests: &[&dyn Testable]){
    serial_println!("Total {} tests", tests.len());
    for test in tests{
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
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

pub fn init(){
    interrupts::init_idt();
    gdt::init();
}
