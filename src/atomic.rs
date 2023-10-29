use core::arch::asm;

use cfg_if::cfg_if;

pub unsafe fn load_barrier() {
    cfg_if! {
        if #[cfg(target_arch = "aarch64")] {
            // FIXME: AArch32 vs AArch64 execution state matters?
            asm!("dmb ishld");
        } else if #[cfg(target_arch="arm")] {
            asm!("dmb ish");
        }
    }
}

pub unsafe fn store_barrier() {
    cfg_if! {
        if #[cfg(any(target_arch = "aarch64", target_arch="arm"))] {
            asm!("dmb ishst");
        }
    }
}

pub unsafe fn barrier() {
    cfg_if! {
        if #[cfg(any(target_arch = "aarch64", target_arch="arm"))] {
            asm!("dmb ish");
        }
    }
}

pub unsafe fn load_sync_barrier() {
    cfg_if! {
        if #[cfg(target_arch = "aarch64")] {
            // FIXME: AArch32 vs AArch64 execution state matters?
            asm!("dsb ishld");
        } else if #[cfg(target_arch="arm")] {
            asm!("dsb ish");
        }
    }
}

pub unsafe fn store_sync_barrier() {
    cfg_if! {
        if #[cfg(any(target_arch = "aarch64", target_arch="arm"))] {
            asm!("dsb ishst");
        }
    }
}

pub unsafe fn sync_barrier() {
    cfg_if! {
        if #[cfg(any(target_arch = "aarch64", target_arch="arm"))] {
            asm!("dsb ish");
        }
    }
}

pub unsafe fn load_uint(ptr: *const u32, ordering: Ordering) -> u32 {
    let addr = ptr as usize;
    #[allow(unused_assignments)]
    let mut value = 0u32;
    match ordering {
        Ordering::Relaxed => load_uint_relaxed(addr, &mut value),
        Ordering::Acquire => load_uint_acquire(addr, &mut value),
        Ordering::SeqCst => load_uint_seqcst(addr, &mut value),
        _ => panic!(),
    }
    value
}

unsafe fn load_uint_relaxed(addr: usize, value: &mut u32) {
    cfg_if! {
        if #[cfg(target_arch = "aarch64")] {
            asm!(
                "ldr {out:w}, [{in}]",
                in = in(reg) addr,
                out = out(reg) *value
            );
        } else if #[cfg(target_arch = "arm")] {
            asm!(
                "ldr {out}, [{in}]",
                in = in(reg) addr,
                out = out(reg) *value
            );
        }
    }
}

unsafe fn load_uint_acquire(addr: usize, value: &mut u32) {
    cfg_if! {
        if #[cfg(target_arch = "aarch64")] {
            asm!(
                "lda {out:w}, [{in}]",
                in = in(reg) addr,
                out = out(reg) *value
            );
        } else if #[cfg(target_arch = "arm")] {
            load_uint_relaxed(addr, value);
            barrier();
        }
    }
}

unsafe fn load_uint_seqcst(addr: usize, value: &mut u32) {
    load_uint_acquire(addr, value);
}

pub unsafe fn store_uint(ptr: *const u32, value: u32, ordering: Ordering) {
    let addr = ptr as usize;
    match ordering {
        Ordering::Relaxed => store_uint_relaxed(addr, value),
        Ordering::Release => store_uint_release(addr, value),
        Ordering::SeqCst => store_uint_seqcst(addr, value),
        _ => panic!(),
    }
}

unsafe fn store_uint_relaxed(addr: usize, value: u32) {
    cfg_if! {
        if #[cfg(target_arch = "aarch64")] {
            asm!(
                "str {in:w}, [{out}]",
                in = in(reg) value,
                out = in(reg) addr
            );
        } else if #[cfg(target_arch = "arm")] {
            asm!(
                "str {in}, [{out}]",
                in = in(reg) value,
                out = in(reg) addr
            );
        }
    }
}

unsafe fn store_uint_release(addr: usize, value: u32) {
    cfg_if! {
        if #[cfg(target_arch = "aarch64")] {
            asm!(
                "stl {in:w}, [{out}]",
                in = in(reg) value,
                out = in(reg) addr
            );
        } else if #[cfg(target_arch = "arm")] {
            barrier();
            store_uint_relaxed(addr, value);
        }
    }
}

unsafe fn store_uint_seqcst(addr: usize, value: u32) {
    store_uint_release(addr, value);
}

pub enum Ordering {
    Relaxed,
    Acquire,
    Release,
    SeqCst,
}

pub struct AtomicUint {
    value: u32,
}

impl AtomicUint {
    pub fn new(initial: u32) -> Self {
        Self { value: initial }
    }

    pub fn load(&self, ordering: Ordering) -> u32 {
        unsafe { load_uint(self.as_ptr(), ordering) }
    }

    pub fn store(&self, value: u32, ordering: Ordering) {
        unsafe { store_uint(self.as_ptr(), value, ordering) }
    }

    fn as_ptr(&self) -> *const u32 {
        &self.value as *const _
    }
}
