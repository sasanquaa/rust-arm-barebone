#![feature(allocator_api)]
#![feature(naked_functions)]
#![feature(negative_impls)]
#![feature(strict_provenance)]
#![feature(asm_const)]
#![feature(const_maybe_uninit_zeroed)]
#![feature(core_intrinsics)]
#![feature(ptr_internals)]
#![no_std]
#![no_main]

use core::arch::asm;

use cfg_if::cfg_if;

mod fs;
mod interrupt;
mod kbox;
mod kmalloc;
mod panic;
mod sync;
mod sysreg;

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
                    "bl {0}",
                    sym main,
                    options(noreturn)
                );
            } else if #[cfg(target_arch="arm")] {
                asm!(
                    "ldr r0, =STACK_TOP",
                    "mov sp, r0",
                    "bl {0}",
                    sym main,
                    options(noreturn)
                );
            }
        }
    }
}

unsafe fn main() -> ! {
    interrupt::init();
    kmalloc::init();
    unreachable!()
}
