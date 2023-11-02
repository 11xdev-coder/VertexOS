use crate::println;

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
                "fart" => {
                    // Attempt to play a test sound and print the result
                    if print_fart() {
                        println!("");
                    } else {
                        println!("Fart [failed]");
                    }
                }
                _ => {
                    // Handle unknown commands
                    println!("Unknown command: {}", cmd);
                }
            }
        } else {
            // Handle the case where there's only one word
            match trimmed_command {
                "fart" => {
                    if print_fart() {
                        println!("");
                    } else {
                        println!("noob");
                    }
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

fn print_fart() -> bool {
    println!(r#"$$$$$$$$\  $$$$$$\  $$$$$$$\  $$$$$$$$\ "#);
    println!(r#"$$  _____|$$  __$$\ $$  __$$\ \__$$  __|"#);
    println!(r#"$$ |      $$ /  $$ |$$ |  $$ |   $$ |   "#);
    println!(r#"$$$$$\    $$$$$$$$ |$$$$$$$  |   $$ |   "#);
    println!(r#"$$  __|   $$  __$$ |$$  __$$<    $$ |   "#);
    println!(r#"$$ |      $$ |  $$ |$$ |  $$ |   $$ |   "#);
    println!(r#"$$ |      $$ |  $$ |$$ |  $$ |   $$ |   "#);
    println!(r#"\__|      \__|  \__|\__|  \__|   \__|   "#);
    true
}
