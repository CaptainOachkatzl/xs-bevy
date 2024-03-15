use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Counter {
    counter: AtomicUsize,
}

impl Counter {
    pub const fn new() -> Self {
        Self {
            counter: AtomicUsize::new(0),
        }
    }

    pub fn tick(&self) {
        self.counter.fetch_add(1, Ordering::SeqCst);
    }

    pub fn count(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}
