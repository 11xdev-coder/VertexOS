use crate::vga_buffer::{self, WRITER};
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicBool, Ordering};

pub static BSOD_ACTIVE: AtomicBool = AtomicBool::new(false);

pub fn execute() {
    panic!("PSOD (Puk Screen Of Death)");
}

pub fn handle_bsod(info: &PanicInfo) {
    let mut writer = WRITER.lock();  

    writer.set_screen_color(vga_buffer::Color::Blue);

    writer.bsod_panic_message(info);

    writer.bsod_title();

    BSOD_ACTIVE.store(true, Ordering::SeqCst); // bsoded
}