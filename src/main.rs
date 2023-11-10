#![feature(allocator_api)]
#![feature(naked_functions)]
#![feature(negative_impls)]
#![feature(strict_provenance)]
#![feature(asm_const)]
#![feature(error_in_core)]
#![feature(const_maybe_uninit_zeroed)]
#![feature(core_intrinsics)]
#![feature(ptr_internals)]
#![feature(ptr_sub_ptr)]
#![no_std]
#![no_main]

use core::arch::asm;
use core::ptr;

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
extern "C" fn boot() {
    const STACK_SIZE: usize = 32768;
    #[no_mangle]
    #[link_section = ".bss.stack"]
    static STACK_SPACE: [u8; STACK_SIZE] = [0; STACK_SIZE];
    #[no_mangle]
    #[link_section = ".bss.stack"]
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
                    sym start,
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

#[cfg(feature = "boot_from_flash")]
#[link_section = ".boot"]
unsafe fn start() {
    main();
}

#[cfg(not(feature = "boot_from_flash"))]
#[link_section = ".boot"]
unsafe fn start() {
    let mut rom_text_start = 0;
    let mut rom_text_size = 0;
    let mut ram_text_start = 0;
    let mut rom_rodata_start = 0;
    let mut rom_rodata_size = 0;
    let mut ram_rodata_start = 0;
    asm!(
        "ldr {out}, =ROM_TEXT_START",
        out = out(reg) rom_text_start
    );
    asm!(
        "ldr {out}, =ROM_TEXT_SIZE",
        out = out(reg) rom_text_size
    );
    asm!(
        "ldr {out}, =RAM_TEXT_START",
        out = out(reg) ram_text_start
    );
    asm!(
        "ldr {out}, =ROM_RODATA_START",
        out = out(reg) rom_rodata_start
    );
    asm!(
        "ldr {out}, =ROM_RODATA_SIZE",
        out = out(reg) rom_rodata_size
    );
    asm!(
        "ldr {out}, =RAM_RODATA_START",
        out = out(reg) ram_rodata_start
    );
    macro_rules! copy {
        ($src:expr, $dst:expr, $count:expr) => {
            let src: *const u8 = $src;
            let dst: *mut u8 = $dst;
            let count: usize = $count;
            let mut i = 0;
            while i < count {
                let src_offset = src.add(i);
                let dst_offset = dst.add(i);
                *dst_offset = *src_offset;
                i = i + 1;
            }
        };
    }
    let src = ptr::from_exposed_addr::<u8>(rom_text_start);
    let dst = ptr::from_exposed_addr_mut::<u8>(ram_text_start);
    let count = rom_text_size;
    copy!(src, dst, count);

    let src = ptr::from_exposed_addr::<u8>(rom_rodata_start);
    let dst = ptr::from_exposed_addr_mut::<u8>(ram_rodata_start);
    let count = rom_rodata_size;
    copy!(src, dst, count);
    main();
}

unsafe fn main() -> ! {
    io::device::uart::init(0x09000000, 24000000);
    itable::init();
    slab::init();
    kprintln!("Hello World!");
    unreachable!()
}
