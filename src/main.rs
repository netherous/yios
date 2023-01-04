#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(yios::test_runner)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;
use x86_64::structures::paging::Translate;
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
    use yios::memory;
    use x86_64::structures::paging::PageTable;
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe {memory::init(phys_mem_offset)};
    let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses{
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?}-> {:?}", virt, phys);
    }

    // let l4_table = unsafe {active_level_4_table(phys_mem_offset)};
    //
    // for(i, entry) in l4_table.iter().enumerate(){
    //     if !entry.is_unused(){
    //         println!("L4 Entry {}: {:?}", i, entry);
    //
    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr: *const PageTable= VirtAddr::new(virt).as_mut_ptr();
    //         let l3_table:&PageTable = unsafe{&*ptr};
    //         for(i,entry) in l3_table.iter().enumerate(){
    //             if !entry.is_unused(){
    //                 println!("  L3 Entry {}: {:?}", i,entry);
    //             }
    //         }
    //     }
    // }
    #[cfg(test)]  
    test_main();

    println!("Testing println function with {}", 9.5);
    // panic!("this is a panic message");
    hlt_loop();
}

