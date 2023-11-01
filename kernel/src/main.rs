#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
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
    print!("> ");

    #[cfg(test)] // only if we ran "cargo test"
    test_main();    
    
    // Sync writer's position
    {        
        let mut writer = WRITER.lock();
        writer.set_column(2); // after > symbol
    }    

    kernel::init_lib(); // start getting input

    loop {

    }
}

// PanicInfo has the file and line where panic happened
// ! -> "never" return type, method never returns (because of the loop)
// creating a panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    hlt_loop();
}

// custom test framework generates custom main method, but we use [no_main] and this method gets ignored
// to fix this, we need to remain the function to something except than main, and then call it in the _start 
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) { 
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run(); 
    }
}

/// TESTS! ------------------------------------------------------------------------------------------------------
// simple test  1
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

// ---------------------------------------------------------
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("{}...\t", core::any::type_name::<T>());
        self();
        println!("[ok]");
    }
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    hlt_loop();
}

// halting
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}


// IMPLEMENTATIONS -------------------------------------------------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = unsafe { *s1.add(i) };
        let b = unsafe { *s2.add(i) };
        if a != b {
            return a as i32 - b as i32;
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    unsafe {
        for i in 0..n {
            *s.add(i) = c as u8;
        }
    }
    s
}

#[no_mangle]
pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        for i in 0..n {
            *dest.add(i) = *src.add(i);
        }
    }
    dest
}
