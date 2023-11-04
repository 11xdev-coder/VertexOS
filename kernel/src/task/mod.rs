use core::{task::{Context, Poll}, future::Future, pin::Pin, sync::atomic::{Ordering, AtomicU64}};
use alloc::boxed::Box;

pub mod simple_executor;
pub mod keyboard;
pub mod executor;

// a little about multitasking:
// Multitasking - ability to manage multiple tasks and manage memory among them
// Preemptive multitasking - CPU makes decisions - how long will any process(or app) use memory
// this gives a more effective usage in system resources
// Cooperative multitasking - Tasks manage switching between processes

pub struct Task {
    id: TaskId,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            id: TaskId::new(), // adds possibility to give task a unique name
            future: Box::pin(future),
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);

impl TaskId {
    fn new() -> Self { // every id is returned exactly once
        static NEXT_ID: AtomicU64 = AtomicU64::new(0); // AtomicU64 to ensure that each ID is assigned once 
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed)) // fetch_add automatically increases the value and return the previous one
    }
}