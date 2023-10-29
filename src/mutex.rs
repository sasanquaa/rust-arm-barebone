use crate::atomic::AtomicUint;

pub struct SpinMutex {
    lock: AtomicUint,
}

impl SpinMutex {
    pub fn new() -> Self {
        Self {
            lock: AtomicUint::new(0),
        }
    }

    pub fn lock(&self) {}
}
