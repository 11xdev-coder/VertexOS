use alloc::collections::BTreeMap;
use spin::Mutex;
use lazy_static::lazy_static;

use crate::println;

// A type alias for test functions
pub type TestFunction = fn() -> ();

// A global registry of test functions
lazy_static! {
    pub static ref TEST_REGISTRY: Mutex<BTreeMap<&'static str, TestFunction>> = Mutex::new(BTreeMap::new());
}

// A function to register a test
pub fn register_test(name: &'static str, test: TestFunction) {
    TEST_REGISTRY.lock().insert(name, test);
}

// A function to run a test by name
pub fn run_test(name: &str) {
    let test_registry = TEST_REGISTRY.lock();
    if let Some(test) = test_registry.get(name) {
        test();
        println!("Test {} [ok]", name);
    } else {
        println!("Test {} not found", name);
    }
}
