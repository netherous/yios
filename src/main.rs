#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    println!("{}",_info);
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
fn test_runner( tests: &[&dyn Fn()]){
    println!("Total {} tests", tests.len());
    for test in tests{
        test();
    }
}

#[test_case]
fn assertion_template(){
    print!("Asserting... ");
    assert_eq!(2,1);
    println!("[passed]");
}
