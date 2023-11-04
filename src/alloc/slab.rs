use core::{alloc, mem, ptr};
use core::alloc::{AllocError, GlobalAlloc, Layout};
use core::mem::MaybeUninit;
use core::ops::Add;
use core::ptr::{NonNull, slice_from_raw_parts_mut};

use crate::debug_assert_call_once;
use crate::sync::SpinMutex;

const BLOCK_COUNT: usize = 8;
const BLOCK_SIZES: [usize; BLOCK_COUNT] = [16, 32, 64, 128, 256, 512, 1024, 2048];

#[global_allocator]
static mut ALLOCATOR: SlabAllocator = SlabAllocator {
    mutex: SpinMutex::new(),
    allocators: unsafe { MaybeUninit::zeroed().assume_init() },
};
const ALLOCATOR_SIZE: usize = 2097152;
#[link_section = ".bss.kmalloc"]
static ALLOCATOR_SPACE: [u8; ALLOCATOR_SIZE] = [0; ALLOCATOR_SIZE];

pub unsafe fn init() {
    debug_assert_call_once!();
    let start_addr = (&ALLOCATOR_SPACE as *const u8).addr();
    let end_addr = start_addr.add(ALLOCATOR_SIZE);
    let capacity = end_addr - start_addr;
    ALLOCATOR.init(start_addr, capacity);
}

#[derive(Copy, Clone, Default)]
pub struct KAllocator;

unsafe impl alloc::Allocator for KAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = unsafe { ALLOCATOR.alloc(layout) };
        if !ptr.is_null() {
            Ok(NonNull::new(slice_from_raw_parts_mut(ptr, layout.size())).unwrap())
        } else {
            Err(AllocError)
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe { ALLOCATOR.dealloc(ptr.as_ptr(), layout) }
    }
}

// TODO: Use improved version of SlabAllocator e.g. SlabAllocator with Magazines
// TODO: Make SlabAllocator expandable by requesting for more memory
struct SlabAllocator {
    mutex: SpinMutex,
    allocators: [Allocator; BLOCK_COUNT],
}

unsafe impl Sync for SlabAllocator {}

unsafe impl Send for SlabAllocator {}

impl SlabAllocator {
    unsafe fn init(&mut self, start_addr: usize, capacity: usize) {
        debug_assert!(
            capacity % BLOCK_COUNT == 0,
            "capacity is not divisible by BLOCK_COUNT"
        );
        let allocator_capacity = capacity / BLOCK_COUNT;
        for (i, allocator) in self.allocators.iter_mut().enumerate() {
            let offset_addr = start_addr + i * allocator_capacity;
            let size = BLOCK_SIZES[i];
            let ptr = allocator as *mut Allocator;
            ptr.write(Allocator::new(offset_addr, size, allocator_capacity));
        }
    }

    fn find_allocator_with_size_locked(&self, size: usize) -> Option<&Allocator> {
        let mut found_allocator = None;
        for allocator in &self.allocators {
            if size <= allocator.block_size {
                found_allocator = Some(allocator);
                break;
            }
        }
        if let Some(allocator) = found_allocator {
            if allocator.free_list.is_null() {
                None
            } else {
                found_allocator
            }
        } else {
            None
        }
    }
}

unsafe impl GlobalAlloc for SlabAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let _ = self.mutex.lock();
        if let Some(allocator) = self.find_allocator_with_size_locked(layout.size()) {
            let current = allocator.get_current_free_list();
            current.write(mem::zeroed());
            current as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let _ = self.mutex.lock();
        let allocator = self.find_allocator_with_size_locked(layout.size()).unwrap();
        let current = ptr as *mut FreeList;
        allocator.set_current_free_list(current);
    }
}

struct Allocator {
    free_list: *mut FreeList,
    block_size: usize,
}

impl Allocator {
    unsafe fn new(offset_addr: usize, size: usize, capacity: usize) -> Self {
        debug_assert!(capacity >= size);
        let offset_start = offset_addr + size;
        let offset_end = offset_addr + capacity;
        let null_ptr_mut = ptr::null_mut::<FreeList>();
        let mut list = ptr::from_exposed_addr_mut::<FreeList>(offset_addr);
        for block_offset in (offset_start..offset_end).step_by(size) {
            let ptr = ptr::from_exposed_addr_mut::<FreeList>(block_offset);
            ptr.write(FreeList { next: null_ptr_mut });
            list.write(FreeList { next: ptr });
            list = ptr;
        }
        Self {
            free_list: ptr::from_exposed_addr_mut(offset_addr),
            block_size: size,
        }
    }

    unsafe fn get_current_free_list(&self) -> *mut FreeList {
        let current = &self.free_list as *const *mut _ as *mut *mut FreeList;
        let next = (*self.free_list).next;
        current.replace(next)
    }

    unsafe fn set_current_free_list(&self, new_current: *mut FreeList) {
        let next = self.free_list;
        let current = &self.free_list as *const *mut _ as *mut *mut FreeList;
        current.write(new_current);
        (&(*new_current).next as *const *mut _ as *mut *mut FreeList).write(next);
    }
}

struct FreeList {
    next: *mut FreeList,
}
