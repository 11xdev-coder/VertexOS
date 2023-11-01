#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#[allow(unused_imports)]

use core::panic::PanicInfo;

pub mod gdt;
pub mod interrupts;
pub mod vga_buffer;
pub mod commands;
pub mod sound;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn init_lib() {
    init();
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
pub fn test_runner(_tests: &[&dyn Fn()]) { 
    // an empty test runner since we got one in the main.rs
    // we need this to resolve the error
}
