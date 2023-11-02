#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

use core::panic::PanicInfo;
use kernel::vga_buffer::WRITER;


// b"string" means to convert the string into bytes
// static means that string will "live" during all the program
// static HELLO: &[u8] = b"GOODBYE HELL";

#[no_mangle] // disable mangling so rust compiler names this method as "_start"
pub extern "C" fn _start() -> ! {
    println!("Basic Kernel Implementation");
    println!("VertexDOS Version 0.1.0");
    
    // Sync writer's position
    {        
        let mut writer = WRITER.lock();
        writer.set_column(2); // after > symbol
    }   

    kernel::init_lib(); // start getting input

    #[cfg(test)] // only if we ran "cargo test"
    test_main();

    println!("Checking state... [ok]");
    print!("> ");
    
    kernel::hlt_loop();
}

// PanicInfo has the file and line where panic happened
// ! -> "never" return type, method never returns (because of the loop)
// creating a panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    kernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}

/// TESTS! ------------------------------------------------------------------------------------------------------
// simple test  1
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
