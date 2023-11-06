#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

extern crate alloc;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use kernel::command_registry;

use kernel::vga_buffer::WRITER;
use alloc::boxed::Box;
use alloc::vec::Vec;
use kernel::{allocator::HEAP_SIZE, test_registry};
use kernel::task::{Task, keyboard, executor::Executor};
use kernel::commands::{bsod::{handle_bsod, self}, fart, echo, test};


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
    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // // write the string `New!` to the screen through the new mapping
    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    // let x = Box::new(41);

    #[cfg(test)] // only if we ran "cargo test"
    test_main();
    
    // registering all tests...
    test_registry::register_test("equal_test", trivial_assertion);
    test_registry::register_test("fail_test", fail_test);
    test_registry::register_test("large_vec", large_vec);
    test_registry::register_test("many_boxes_long_lived", many_boxes_long_lived);
    test_registry::register_test("simple_alloc", simple_allocation);
    test_registry::register_test("many_boxes", many_boxes);
    test_registry::register_test("simple_println", println_simple);
    test_registry::register_test("many_println", println_many);

    // registering commands
    command_registry::register_command("bsod", bsod::execute);
    command_registry::register_command("fart", fart::execute);
    command_registry::register_command_with_args("echo", echo::execute);
    command_registry::register_command_with_args("test", test::execute);

    println!("Checking state... [ok]");
    print!("> ");

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypress()));
    executor.run();
}

// PanicInfo has the file and line where panic happened
// ! -> "never" return type, method never returns (because of the loop)
// creating a panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    handle_bsod(info);
    kernel::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}

/// TESTS! ------------------------------------------------------------------------------------------------------
fn trivial_assertion() {
    assert_eq!(1, 1);
}

fn fail_test() {
    assert_eq!(0, 1);
}

// some heap allocation tests
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

fn many_boxes_long_lived() {
    let long_lived = Box::new(1); // new
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*long_lived, 1); // new
}

fn simple_allocation() {
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);
}

fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

fn println_simple() {
    println!("test_println_simple output");
}

fn println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}


// async fn async_number() -> u32 {
//     42
// }

// async fn async_test() {
//     let number = async_number().await;
//     println!("async test: {number}");
// }
