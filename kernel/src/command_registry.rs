use alloc::collections::BTreeMap;
use spin::Mutex;
use lazy_static::lazy_static;
use crate::println;

// type alias for command with arguments
pub type CommandWithArgs = fn(&str);

// type alias for command without arguments
pub type Command = fn();

// enum with command variants
pub enum CommandFunction {
    WithArgs(CommandWithArgs),
    NoArgs(Command),
}

// global registry for commands
lazy_static! {
    pub static ref COMMAND_REGISTRY: Mutex<BTreeMap<&'static str, CommandFunction>> = Mutex::new(BTreeMap::new());
}

// A function to register a command with arguments
pub fn register_command_with_args(name: &'static str, command: CommandWithArgs) {
    COMMAND_REGISTRY.lock().insert(name, CommandFunction::WithArgs(command));
}

// A function to register a command without arguments
pub fn register_command(name: &'static str, command: Command) {
    COMMAND_REGISTRY.lock().insert(name, CommandFunction::NoArgs(command));
}

pub fn run_command(command_line: &str) {
    let trimmed_command_line = command_line.trim();
    let mut parts = trimmed_command_line.splitn(2, ' ');
    let command_name = parts.next().unwrap_or("");
    let args = parts.next();

    let command_registry = COMMAND_REGISTRY.lock();
    if let Some(command_function) = command_registry.get(command_name) {
        match command_function {
            CommandFunction::WithArgs(function) => {
                if let Some(arguments) = args {
                    function(arguments);
                } else {
                    println!("Error: {} requires arguments", command_name);
                }
            },
            CommandFunction::NoArgs(function) => {
                if args.is_none() {
                    function();
                } else {
                    println!("Error: {} does not accept arguments", command_name);
                }
            },
        }
    } else {
        println!("{} not found", command_name);
    }
}