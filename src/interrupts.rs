use core::char;

use crate::pic8259::ATPic;
use crate::vga_buffer::_clear_byte;
use crate::{gdt, println};
use lazy_static::lazy_static;
use spin;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

pub const PIC1_OFFSET: u8 = 32;
pub const PIC2_OFFSET: u8 = 40;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn u8(self) -> u8 {
        self as u8
    }
    fn usize(self) -> usize {
        self as usize
    }
}

pub static PICS: spin::Mutex<ATPic> =
    spin::Mutex::new(unsafe { ATPic::new(PIC1_OFFSET, PIC2_OFFSET) });

lazy_static! {
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
        idt[InterruptIndex::Keyboard.usize()].set_handler_fn(keyboard_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt
    };
}

//exception vector maps from 0x00 to 0x1E which is 30
//
pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use crate::hlt_loop;
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Access Address: {:?}", Cr2::read());
    println!("error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    hlt_loop();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    // println!("timer_handler\n{:#?}", _stack_frame);
    use crate::print;
    // print!(".");
    unsafe {
        PICS.lock().handle_eoi(InterruptIndex::Timer.u8());
    }
}

extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    use crate::print;
    use pc_keyboard::*;
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    }
    let mut keyboard = KEYBOARD.lock();
    const PS2_IO_PORT: u16 = 0x60;
    let mut port: Port<u8> = Port::new(PS2_IO_PORT);
    let scancode: u8 = unsafe { port.read() };
    let backspace: char = 0x08.into();
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => match character {
                    //backspace unicode
                    '\u{08}' => {
                        _clear_byte();
                    }
                    _ => print!("{}", character),
                },
                DecodedKey::RawKey(key) => {
                    print!("{:?}", key)
                }
            }
        }
    }

    unsafe {
        PICS.lock().handle_eoi(InterruptIndex::Keyboard.u8());
    }
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _: u64) -> ! {
    println!("EXCEPTION DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
