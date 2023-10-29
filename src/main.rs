#![feature(allocator_api)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]
#![allow(named_asm_labels)]
#![no_std]
#![no_main]

use core::alloc::GlobalAlloc;
use core::arch::asm;
use core::intrinsics::atomic_load_acquire;

use cfg_if::cfg_if;

mod atomic;
mod fs;
mod kmalloc;
mod mutex;
mod panic;

extern "C" {
    static STACK_TOP: usize;
}

#[naked]
#[no_mangle]
extern "C" fn start() {
    unsafe {
        cfg_if! {
            if #[cfg(target_arch="aarch64")] {
                asm!(
                    "ldr x0, ={0}",
                    "mov sp, x0",
                    "bl {1}",
                    sym STACK_TOP,
                    sym kernel_main,
                    options(noreturn)
                );
            } else if #[cfg(target_arch="arm")] {
                asm!(
                    "ldr r0, ={0}",
                    "mov sp, r0",
                    "bl {1}",
                    sym STACK_TOP,
                    sym kernel_main,
                    options(noreturn)
                );
            }
        }
    }
}

#[no_mangle]
fn kernel_main() {
    // let a = 1337u32;
    // let mut b = unsafe { atomic::load_uint(&a as *const _, Ordering::Relaxed) };
    // b += 1;
    let mut a = 0u32;
    unsafe {
        // atomic_store_seqcst(&mut a as *mut _, 1u32);
        atomic_load_acquire(&a as *const _);
    }
}
