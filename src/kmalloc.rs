use core::alloc::{GlobalAlloc, Layout};
use core::cell::Cell;
use core::mem::{MaybeUninit, transmute};

const BLOCK_COUNT: usize = 4;
const BLOCK_SIZES: [usize; BLOCK_COUNT] = [16, 32, 64, 128];

pub struct SlabAllocator {
    allocators: [Allocator; BLOCK_COUNT],
}

struct Allocator {
    free_list: FreeList,
    size: usize,
}

struct Block {
    data: *const u8,
}

struct FreeList {
    next: Cell<Option<Block>>,
}

// static SLAB_ALLOCATOR_INIT: OnceRef<SlabAllocator> = OnceRef::new();
// #[global_allocator]
// static SLAB_ALLOCATOR: SlabAllocator = *SLAB_ALLOCATOR_INIT.get_or_init(|| {
//     &SlabAllocator::new()
// });

impl SlabAllocator {
    pub fn new() -> Self {
        let mut allocators = unsafe { MaybeUninit::<[MaybeUninit<Allocator>; BLOCK_COUNT]>::uninit().assume_init() };
        for (i, allocator) in allocators.iter_mut().enumerate() {
            *allocator = MaybeUninit::new(Allocator::new(BLOCK_SIZES[i]));
        }
        Self {
            allocators: unsafe { transmute(allocators) }
        }
    }
}

unsafe impl Sync for SlabAllocator {}

unsafe impl Send for SlabAllocator {}

unsafe impl GlobalAlloc for SlabAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        0 as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {}
}

impl Allocator {
    fn new(size: usize) -> Self {
        Self {
            free_list: FreeList { next: Cell::new(None) },
            size,
        }
    }
}

