#[allow(unused_imports)]
use core::arch::asm;

use cfg_if::cfg_if;

use crate::debug_assert_call_once;

pub unsafe fn init() {
    debug_assert_call_once!();
    cfg_if! {
        if #[cfg(any(target_arch = "aarch64"))] {
            init_table_vbar();
        }
    }
}

#[cfg(target_arch = "aarch64")]
unsafe fn init_table_vbar() {
    asm!(
        "ldr {1}, ={0}",
        "msr VBAR_EL1, {1}",
        "dsb ish",
        sym elx_exception_table,
        out(reg) _,
    );
}

#[naked]
#[link_section = ".text.itable"]
#[cfg(target_arch = "aarch64")]
#[allow(undefined_naked_function_abi)]
unsafe fn elx_exception_table() {
    asm!(
        "b {el0_sync}", // start of EL0
        ".skip 124",
        "b {el0_irq}",
        ".skip 124",
        "b {el0_fiq}",
        ".skip 124",
        "b {el0_serror}",
        ".skip 124", // end of EL0
        "b {elx_sync}", // start of ELx, x > 0
        ".skip 124",
        "b {elx_irq}",
        ".skip 124",
        "b {elx_fiq}",
        ".skip 124",
        "b {elx_serror}",
        ".skip 124", // end of ELx, x > 0
        "b {elx_lower_aarch64_sync}",   // start of Aarch64 Lower Exception level
        ".skip 124",
        "b {elx_lower_aarch64_irq}",
        ".skip 124",
        "b {elx_lower_aarch64_fiq}",
        ".skip 124",
        "b {elx_lower_aarch64_serror}",
        ".skip 124", // end of Aarch64 Lower Exception level
        "b {elx_lower_aarch32_sync}",   // start of Aarch32 Lower Exception level
        ".skip 124",
        "b {elx_lower_aarch32_irq}",
        ".skip 124",
        "b {elx_lower_aarch32_fiq}",
        ".skip 124",
        "b {elx_lower_aarch32_serror}",
        ".skip 124", // end of Aarch32 Lower Exception level,
        el0_sync = sym el0_handle_sync_exception,
        el0_irq = sym el0_handle_irq_exception,
        el0_fiq = sym el0_handle_fiq_exception,
        el0_serror = sym el0_handle_serror_exception,
        elx_sync = sym elx_handle_sync_exception,
        elx_irq = sym elx_handle_irq_exception,
        elx_fiq = sym elx_handle_fiq_exception,
        elx_serror = sym elx_handle_serror_exception,
        elx_lower_aarch64_sync = sym elx_lower_aarch64_handle_sync_exception,
        elx_lower_aarch64_irq = sym elx_lower_aarch64_handle_irq_exception,
        elx_lower_aarch64_fiq = sym elx_lower_aarch64_handle_fiq_exception,
        elx_lower_aarch64_serror = sym elx_lower_aarch64_handle_serror_exception,
        elx_lower_aarch32_sync = sym elx_lower_aarch32_handle_sync_exception,
        elx_lower_aarch32_irq = sym elx_lower_aarch32_handle_irq_exception,
        elx_lower_aarch32_fiq = sym elx_lower_aarch32_handle_fiq_exception,
        elx_lower_aarch32_serror = sym elx_lower_aarch32_handle_serror_exception,
        options(noreturn)
    );
}

#[cfg(target_arch = "aarch64")]
unsafe fn el0_handle_sync_exception() {
    // FIXME: handle sync exception from ESR_EL1
    loop {}
}

#[cfg(target_arch = "aarch64")]
unsafe fn el0_handle_irq_exception() {
    // FIXME: handle irq exception from PSTATE
    loop {}
}

#[cfg(target_arch = "aarch64")]
unsafe fn el0_handle_fiq_exception() {
    // FIXME: handle irq exception from PSTATE
    loop {}
}

#[cfg(target_arch = "aarch64")]
unsafe fn el0_handle_serror_exception() {
    // FIXME: handle irq exception from PSTATE
    loop {}
}

#[cfg(target_arch = "aarch64")]
unsafe fn elx_handle_sync_exception() {
    // FIXME: handle sync exception from ESR_EL1
    loop {}
}

#[cfg(target_arch = "aarch64")]
unsafe fn elx_handle_irq_exception() {
    // FIXME: handle irq exception from PSTATE
    loop {}
}

#[cfg(target_arch = "aarch64")]
unsafe fn elx_handle_fiq_exception() {
    // FIXME: handle irq exception from PSTATE
    loop {}
}

#[cfg(target_arch = "aarch64")]
unsafe fn elx_handle_serror_exception() {
    // FIXME: handle irq exception from PSTATE
    loop {}
}

#[cfg(target_arch = "aarch64")]
unsafe fn elx_lower_aarch64_handle_sync_exception() {
    // FIXME: handle sync exception from ESR_EL1
    loop {}
}

#[cfg(target_arch = "aarch64")]
unsafe fn elx_lower_aarch64_handle_irq_exception() {
    // FIXME: handle irq exception from PSTATE
    loop {}
}

#[cfg(target_arch = "aarch64")]
unsafe fn elx_lower_aarch64_handle_fiq_exception() {
    // FIXME: handle irq exception from PSTATE
    loop {}
}

#[cfg(target_arch = "aarch64")]
unsafe fn elx_lower_aarch64_handle_serror_exception() {
    // FIXME: handle irq exception from PSTATE
    loop {}
}

#[cfg(target_arch = "aarch64")]
unsafe fn elx_lower_aarch32_handle_sync_exception() {
    // FIXME: handle sync exception from ESR_EL1
    loop {}
}

#[cfg(target_arch = "aarch64")]
unsafe fn elx_lower_aarch32_handle_irq_exception() {
    // FIXME: handle irq exception from PSTATE
    loop {}
}

#[cfg(target_arch = "aarch64")]
unsafe fn elx_lower_aarch32_handle_fiq_exception() {
    // FIXME: handle irq exception from PSTATE
    loop {}
}

#[cfg(target_arch = "aarch64")]
unsafe fn elx_lower_aarch32_handle_serror_exception() {
    // FIXME: handle irq exception from PSTATE
    loop {}
}
