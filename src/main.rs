#![feature(allocator_api)]
#![feature(naked_functions)]
#![feature(negative_impls)]
#![feature(strict_provenance)]
#![feature(asm_const)]
#![feature(error_in_core)]
#![feature(const_maybe_uninit_zeroed)]
#![feature(core_intrinsics)]
#![feature(ptr_internals)]
#![no_std]
#![no_main]

use core::arch::asm;

use cfg_if::cfg_if;

use crate::alloc::slab;
use crate::sys::itable;

mod alloc;
mod debug;
mod io;
mod panic;
mod sync;
mod sys;

#[naked]
#[no_mangle]
#[link_section = ".boot"]
#[cfg(feature = "boot_from_flash")]
extern "C" fn boot() {
    unsafe {
        asm!(
            "b {0}",
            sym start,
            options(noreturn)
        );
    }
}

#[naked]
#[no_mangle]
#[link_section = ".boot"]
#[cfg(not(feature = "boot_from_flash"))]
extern "C" fn boot() {
    extern "C" {
        static ROM_TEXT_START: usize;
        static ROM_TEXT_END: usize;
        static RAM_TEXT_START: usize;
    }
    unsafe {
        cfg_if! {
            if #[cfg(target_arch="aarch64")] {
                asm!(
                    "ldr x0, =ROM_TEXT_START",
                    "ldr x1, =ROM_TEXT_END",
                    "ldr x2, =RAM_TEXT_START",
                    "ldr x3, =start",
                    "sub x3, x3, x0",
                    "add x3, x3, x2",
                    "0:",
                    "cmp x0, x1",
                    "bge 1f",
                    "ldr x4, [x0], #8",
                    "str x4, [x2], #8",
                    "blt 0b",
                    "1:",
                    "br x3",
                    options(noreturn)
                );
            } else if #[cfg(target_arch="arm")] {
                asm!(
                    "ldr r0, =ROM_TEXT_START",
                    "ldr r1, =ROM_TEXT_END",
                    "ldr r2, =RAM_TEXT_START",
                    "ldr r3, =start",
                    "sub r3, r3, r0",
                    "add r3, r3, r2",
                    "0:",
                    "cmp r0, r1",
                    "bge 1f",
                    "ldr r4, [r0], #8",
                    "str r4, [r2], #8",
                    "blt 0b",
                    "1:",
                    "bx r3", // FIXME: seems to matter about which aarch state is encoded
                    options(noreturn)
                );
            }
        }
    }
}

#[naked]
#[no_mangle]
extern "C" fn start() {
    const STACK_SIZE: usize = 32768;
    #[link_section = ".bss.stack"]
    #[no_mangle]
    static STACK_SPACE: [u8; STACK_SIZE] = [0; STACK_SIZE];
    #[link_section = ".bss.stack"]
    #[no_mangle]
    static STACK_TOP: [u8; 0] = [0; 0];
    unsafe {
        cfg_if! {
            if #[cfg(target_arch="aarch64")] {
                asm!(
                    "ldr x0, =0",
                    "orr x0, x0, #3145728", // allow SIMD & FP for EL0/1
                    "msr CPACR_EL1, x0",
                    "ldr x0, =STACK_TOP",
                    "mov sp, x0",
                    "dsb ish",
                    "b {0}",
                    sym main,
                    options(noreturn)
                );
            } else if #[cfg(target_arch="arm")] {
                // FIXME: SIMD & FP?
                asm!(
                    "ldr r0, =STACK_TOP",
                    "mov sp, r0",
                    "b {0}",
                    sym main,
                    options(noreturn)
                );
            }
        }
    }
}

unsafe fn main() -> ! {
    io::device::uart::init(0x09000000, 24000000);
    itable::init();
    slab::init();
    kprintln!("Hello World!");
    unreachable!()
}
