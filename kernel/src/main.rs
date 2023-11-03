#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

extern crate alloc;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use kernel::vga_buffer::WRITER;
use alloc::boxed::Box;


// b"string" means to convert the string into bytes
// static means that string will "live" during all the program
// static HELLO: &[u8] = b"GOODBYE HELL";

entry_point!(kernel_main); 
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use kernel::allocator;
    use kernel::memory::{self, BootInfoFrameAllocator};
    use x86_64::{structures::paging::Page, VirtAddr};
    
    println!("Basic Kernel Implementation");
    println!("VertexDOS Version 0.1.0");
    
    // Sync writer's position
    {        
        let mut writer = WRITER.lock();
        writer.set_column(2); // after > symbol
    }   

    kernel::init(); // start getting input

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // new
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let x = Box::new(41);
    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    let x = Box::new(41);

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

fn trivial_assertion() {
    assert_eq!(1, 1);
}
