use std::sync::atomic::{AtomicU64, Ordering};

pub struct DelayState {
    value: AtomicU64,
}

impl DelayState {
    pub fn new() -> Self {
        Self {
            value: AtomicU64::new(0),
        }
    }

    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }

    pub fn set(&self, v: u64) {
        self.value.store(v, Ordering::Relaxed);
    }
}
