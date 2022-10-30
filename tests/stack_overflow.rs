#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use yios::{serial_print, gdt::DOUBLE_FAULT_IST_INDEX, exit_qemu, QemuExitCode};
use x86_64::structures::idt;

#[no_mangle]
pub extern "C" fn _start() -> !{
    serial_print!("stack_overflow::stack_overflow...\t");
    yios::gdt::init();
    ini_test_gdt();

    stack_overflow();

    panic!("Execution continued after stack overflow!");
}

#[allow(unconditional_recursion)]
fn stack_overflow(){
    stack_overflow();
    volatile::Volatile::new(0).read();
}

lazy_static::lazy_static!{
    static ref TEST_IDT: idt::InterruptDescriptorTable = {
        let mut idt = idt::InterruptDescriptorTable::new();
        unsafe{
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

fn ini_test_gdt(){
    TEST_IDT.load();    
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: idt::InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_print!("[passed]\n");
    exit_qemu(QemuExitCode::Success);
    loop{}
}

#[panic_handler]
fn panic (_info: &PanicInfo) -> !{
    yios::test_panic_handler(_info)
}
