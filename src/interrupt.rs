use core::arch::asm;

use cfg_if::cfg_if;

pub unsafe fn init() {
    cfg_if! {
        if #[cfg(any(target_arch = "aarch64"))] {
            init_table_vbar();
        }
    }
}

#[cfg(target_arch = "aarch64")]
unsafe fn init_table_vbar() {
    let vbar = (elx_exception_table as *const ()).addr();
    asm!("msr VBAR_EL1, {0}", in(reg) vbar);
    asm!("dsb ish");
}

#[naked]
#[link_section = ".text.table"]
#[cfg(target_arch = "aarch64")]
#[allow(undefined_naked_function_abi)]
unsafe fn elx_exception_table() {
    asm!(
        ".skip 4", // start of EL0
        ".skip 124",
        ".skip 4",
        ".skip 124",
        ".skip 4",
        ".skip 124",
        ".skip 4",
        ".skip 124",
        "bl elx_handle_sync_exception", // start of ELx, x > 0
        ".skip 124",
        "bl elx_handle_irq_exception",
        ".skip 124",
        "bl elx_handle_fiq_exception",
        ".skip 124",
        "bl elx_handle_seerror_exception",
        ".skip 124", // end of ELx, x > 0
        ".skip 4",   // start of Aarch64 Lower Exception level
        ".skip 124",
        ".skip 4",
        ".skip 124",
        ".skip 4",
        ".skip 124",
        ".skip 4",
        ".skip 124", // end of Aarch64 Lower Exception level
        ".skip 4",   // start of Aarch32 Lower Exception level
        ".skip 124",
        ".skip 4",
        ".skip 124",
        ".skip 4",
        ".skip 124",
        ".skip 4",
        ".skip 124", // end of Aarch32 Lower Exception level
        options(noreturn)
    );
}

#[no_mangle]
#[cfg(target_arch = "aarch64")]
unsafe fn elx_handle_sync_exception() {
    // FIXME: handle sync exception from ESR_EL1
    loop {}
}

#[no_mangle]
#[cfg(target_arch = "aarch64")]
unsafe fn elx_handle_irq_exception() {
    // FIXME: handle irq exception from PSTATE
    loop {}
}

#[no_mangle]
#[cfg(target_arch = "aarch64")]
unsafe fn elx_handle_fiq_exception() {
    // FIXME: handle irq exception from PSTATE
    loop {}
}

#[no_mangle]
#[cfg(target_arch = "aarch64")]
unsafe fn elx_handle_seerror_exception() {
    // FIXME: handle irq exception from PSTATE
    loop {}
}
