use core::cell::UnsafeCell;
use core::hint::spin_loop;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

pub struct SpinMutex {
    locked: AtomicBool,
}

unsafe impl Sync for SpinMutex {}

unsafe impl Send for SpinMutex {}

impl SpinMutex {
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) -> SpinMutexGuard {
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            spin_loop()
        }
        SpinMutexGuard { mutex: self }
    }
}

pub struct SpinMutexGuard<'a> {
    mutex: &'a SpinMutex,
}

impl !Send for SpinMutexGuard<'_> {}

unsafe impl Sync for SpinMutexGuard<'_> {}

impl Drop for SpinMutexGuard<'_> {
    fn drop(&mut self) {
        self.mutex.locked.store(false, Ordering::Release)
    }
}

pub struct LazyStatic<T, F = fn() -> T> {
    mutex: SpinMutex,
    value: UnsafeCell<Option<T>>,
    factory: F,
}

unsafe impl<T: Sync> Sync for LazyStatic<T> {}

#[allow(suspicious_auto_trait_impls)]
unsafe impl<T: Send> Send for LazyStatic<T> {}

impl<T> LazyStatic<T> {
    pub const fn new(factory: fn() -> T) -> Self {
        Self {
            mutex: SpinMutex::new(),
            value: UnsafeCell::new(None),
            factory,
        }
    }

    pub fn into_inner(self) -> T {
        unsafe { self.ensure_initialized() };
        self.value.into_inner().unwrap()
    }

    unsafe fn ensure_initialized(&self) {
        if (*self.value.get()).is_some() {
            return;
        }
        let _ = self.mutex.lock();
        if (*self.value.get()).is_none() {
            *self.value.get() = Some((self.factory)());
        }
    }

    fn get(&self) -> &T {
        unsafe {
            self.ensure_initialized();
            (*self.value.get()).as_ref().unwrap()
        }
    }

    fn get_mut(&mut self) -> &mut T {
        unsafe {
            self.ensure_initialized();
            (*self.value.get()).as_mut().unwrap()
        }
    }
}

impl<T> Deref for LazyStatic<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> DerefMut for LazyStatic<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}
