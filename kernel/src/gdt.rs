use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0; // index of interrupt stack table entry for the DOUBLE FAULT Interrupt

// TSS is a data structure that contains info about kernel stack and other tasks that needed to switch between tasks
lazy_static! {
    static ref TSS: TaskStateSegment = { // creating task state segment for our kernel
        let mut tss = TaskStateSegment::new();
        // set the interrupt stack table entry for the DOUBLE FAULT Interrupt.
        // DOUBLE FAULT Interrupt can occur at any time, even when kernel is handling another interrupt
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = { // set interrupt stack table to DOUBLE FAULT Interrupt
            const STACK_SIZE: usize = 4096 * 5; // 5 KiB in size (large enough)
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK }); // creating a virtual address for the top of the kernel stack (unsafe { &STACK } -> 
                // is a direct pointer to the top of the kernel stack)
            let stack_end = stack_start + STACK_SIZE; // assign the bottom of the kernel stack to DOUBLE FAULT Interrupt
            stack_end
        };
        tss // return TSS object
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = { // global descriptor table for our kernel
        let mut gdt = GlobalDescriptorTable::new(); // create a new GDT object
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment()); // add an entry for kernel code segment
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS)); // add an entry for TSS

        // return tuple that contains GDT and Selectors
        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
            },
        )
    };
}

struct Selectors {
    code_selector: SegmentSelector, // SegmentSelector is a value that CPU uses to identify a specific segment in GDT
    tss_selector: SegmentSelector,
} // this contains the segment selectors for the kernel code segment and the TSS

pub fn init() { // loads GDT and TSS into CPU
    use x86_64::instructions::segmentation::{Segment, CS};
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}