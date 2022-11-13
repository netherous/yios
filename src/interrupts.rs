use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use crate::{println, gdt};
use spin;
use crate::pic8259::ATPic;

pub const PIC1_OFFSET: u8 = 32;
pub const PIC2_OFFSET: u8 = 40;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex{
    Timer = PIC1_OFFSET,
}

impl InterruptIndex{
    fn u8(self)->u8{
        self as u8
    }
    fn usize(self)->usize{
        self as usize
    }
}

pub static PICS: spin::Mutex<ATPic>=
    spin::Mutex::new(unsafe{ATPic::new(PIC1_OFFSET,PIC2_OFFSET)});

lazy_static!{
    //IDT which contains the mapping between each Interrupts and its handler
    //need to initiallized
    static ref IDT:InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe{
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer.usize()].set_handler_fn(timer_handler);
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

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame)
{
    // println!("timer_handler\n{:#?}", stack_frame);
    use crate::print;
    print!(".");
    unsafe{
       PICS.lock().handle_eoi(InterruptIndex::Timer.u8());
    }
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
