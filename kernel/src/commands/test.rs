use crate::test_registry;

pub fn execute(args: &str) {
    handle_test_command(args);
}

fn handle_test_command(test_file: &str) {
    test_registry::run_test(test_file);
}