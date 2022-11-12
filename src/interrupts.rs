use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use crate::{println, gdt};

lazy_static!{

    static ref IDT:InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe{
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

//exception vector maps from 0x00 to 0x1E which is 30
//
pub fn init_idt(){
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame)
{
    println!("EXCEPTION BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler
(stack_frame: InterruptStackFrame, _:u64)->!{

    println!("EXCEPTION DOUBLE FAULT\n{:#?}", stack_frame);
    loop{}
}

#[test_case]
fn test_breakpoint_exception(){
    x86_64::instructions::interrupts::int3();
}