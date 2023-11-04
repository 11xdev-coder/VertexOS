use crate::{println, test_registry, vga_buffer};
use x86_64::instructions::interrupts::{self, enable_and_hlt};
use core::sync::atomic::{Ordering, AtomicBool};

pub static BSOD_ACTIVE: AtomicBool = AtomicBool::new(false);

pub fn handle_command(command_bytes: &[u8]) {
    // Convert the byte slice to a string
    if let Ok(command_str) = core::str::from_utf8(command_bytes) {
        // Trim the command string to remove leading/trailing whitespace
        let trimmed_command = command_str.trim();

        // Find the first space to separate the command from its arguments
        if let Some(first_space_index) = trimmed_command.find(' ') {
            let (cmd, args) = trimmed_command.split_at(first_space_index);
            match cmd {
                "echo" => {
                    // Print everything after the first space
                    println!("{}", &args[1..]);
                }
                "test" => {
                    handle_test_command(&args[1..]);
                }
                "bsod" => {
                    println!("{cmd} does not accept any arguments");
                }
                "fart" => {
                    print_fart();
                }
                _ => {
                    // Handle unknown commands
                    println!("Unknown command: {}", cmd);
                }
            }
        } else {
            // Handle the case where there's only one word
            match trimmed_command {
                "test" => {
                    println!("Usage: test <test_file_name>");
                }
                "bsod" => { // suggestion by toxxxik
                    handle_bsod();
                }
                "fart" => {
                    print_fart();
                }
                _ => {
                    println!("Unknown command: {}", trimmed_command);
                }
            }
        }
    } else {
        println!("Failed to parse command");
    }
}

fn print_fart() {
    println!(r#"$$$$$$$$\  $$$$$$\  $$$$$$$\  $$$$$$$$\ "#);
    println!(r#"$$  _____|$$  __$$\ $$  __$$\ \__$$  __|"#);
    println!(r#"$$ |      $$ /  $$ |$$ |  $$ |   $$ |   "#);
    println!(r#"$$$$$\    $$$$$$$$ |$$$$$$$  |   $$ |   "#);
    println!(r#"$$  __|   $$  __$$ |$$  __$$<    $$ |   "#);
    println!(r#"$$ |      $$ |  $$ |$$ |  $$ |   $$ |   "#);
    println!(r#"$$ |      $$ |  $$ |$$ |  $$ |   $$ |   "#);
    println!(r#"\__|      \__|  \__|\__|  \__|   \__|   "#);
}

fn handle_test_command(test_file: &str) {
    test_registry::run_test(test_file);
}

fn handle_bsod() {
    
    interrupts::disable(); // disable input
    vga_buffer::set_screen_color(vga_buffer::Color::Blue);
    vga_buffer::print_bsod_message();

    {
        let mut writer = vga_buffer::WRITER.lock();
        writer.return_to_default_color();
    }  

    BSOD_ACTIVE.store(true, Ordering::SeqCst);
}
