use core::alloc::{Allocator, Layout};
use core::ops::{Deref, DerefMut};
use core::ptr::{NonNull, Unique};

use crate::kmalloc::KmallocAllocator;

pub struct KBox<T: ?Sized, A: Allocator = KmallocAllocator> {
    ptr: Unique<T>,
    allocator: A,
    layout: Layout,
}

impl<T> KBox<T> {
    pub fn new(value: T) -> Self {
        let allocator = KmallocAllocator;
        let layout = Layout::new::<T>();
        let ptr = allocator.allocate(layout).unwrap().cast::<T>();
        unsafe {
            ptr.as_ptr().write(value);
        }
        Self {
            ptr: ptr.into(),
            allocator,
            layout,
        }
    }
}

impl<T: ?Sized, A: Allocator> Deref for KBox<T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized, A: Allocator> DerefMut for KBox<T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: ?Sized, A: Allocator> Drop for KBox<T, A> {
    fn drop(&mut self) {
        unsafe {
            self.allocator
                .deallocate(NonNull::from(self.ptr).cast(), self.layout);
        }
    }
}
