#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(yios::test_runner)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;
use yios::*;
use bootloader::{BootInfo, entry_point};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    println!("{}",_info);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    yios::test_panic_handler(_info);
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {

    println!("TEMOC{}", "! Loves the world, and Lets go !");

    yios::init();

    use x86_64::VirtAddr;
    use yios::memory::active_level_4_table;
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe {active_level_4_table(phys_mem_offset)};

    for(i, entry) in l4_table.iter().enumerate(){
        if !entry.is_unused(){
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }
    #[cfg(test)]  
    test_main();

    println!("Testing println function with {}", 9.5);
    // panic!("this is a panic message");
    hlt_loop();
}

