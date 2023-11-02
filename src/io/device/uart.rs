#![allow(dead_code)]

use core::{hint, ptr};
use core::ops::DerefMut;
use core::ptr::from_exposed_addr;

use crate::debug_assert_call_once;
use crate::sync::LazyStatic;

static mut DEVICE: LazyStatic<Uart> = LazyStatic::new(|| Uart {
    registers: ptr::null_mut(),
});

pub unsafe fn init(register_base: usize, base_clock: u32) {
    debug_assert_call_once!();
    let device = DEVICE.deref_mut();
    let ptr = &device.registers as *const *mut _ as *mut *const Registers;
    ptr.write(from_exposed_addr(register_base));
    device.init(base_clock);
}

#[repr(C)]
struct Registers {
    data: u32,
    receive_status_or_error_clear: u32,
    res0: [u32; 4],
    flag: u32,
    res1: u32,
    low_power_counter: u32,
    baud_rate_i: u32,
    baud_rate_f: u32,
    line_control: u32,
    control: u32,
    fifo_level: u32,
    interrupt_mask_set_or_clear: u32,
    masked_interrupt_status: u32,
    interrupt_clear: u32,
    dma_control: u32,
}

struct Control;

impl Control {
    const ENABLE: u32 = 1 << 0;
    const TRANSMIT: u32 = 1 << 8;
    const RECEIVE: u32 = 1 << 9;
}

struct Flag;

impl Flag {
    const TRANSMIT_FULL: u32 = 1 << 5;
    const RECEIVE_EMPTY: u32 = 1 << 6;
}

struct LineControl;

impl LineControl {
    const FIFO: u32 = 1 << 4;
    const WORD_LEN_8: u32 = 3 << 5;
}

struct Uart {
    registers: *mut Registers,
}

impl Uart {
    fn init(&mut self, base_clock: u32) {
        self.baud_rate(230400, base_clock);
        self.enable();
    }

    fn line_control(&mut self) {
        let reg = self.reg_mut();
        reg.line_control = LineControl::FIFO | LineControl::WORD_LEN_8;
    }

    fn baud_rate(&mut self, baud_rate: u32, base_clock: u32) {
        let reg = self.reg_mut();
        let baud_rate_divisor = base_clock / (16 * baud_rate);
        reg.baud_rate_i = baud_rate_divisor & 0x3F;
        reg.baud_rate_f = (baud_rate_divisor >> 6) & 0xFF;
    }

    fn enable(&mut self) {
        let reg = self.reg_mut();
        reg.control = Control::ENABLE | Control::TRANSMIT | Control::RECEIVE;
    }

    fn receive(&mut self) -> u8 {
        let reg = self.reg_mut();
        #[allow(clippy::while_immutable_condition)]
        while reg.flag & Flag::RECEIVE_EMPTY != 0 {
            hint::spin_loop() // FIXME
        }
        (reg.data & 0xFF) as u8
    }

    fn send(&mut self, byte: u8) {
        let reg = self.reg_mut();
        #[allow(clippy::while_immutable_condition)]
        while reg.flag & Flag::TRANSMIT_FULL != 0 {
            hint::spin_loop() // FIXME
        }
        reg.data = byte as u32;
    }

    fn print_str(&mut self, str: &[u8]) {
        for c in str {
            if *c == b'\n' {
                self.send(b'\r');
            }
            self.send(*c);
        }
    }

    fn reg_mut(&mut self) -> &mut Registers {
        unsafe { &mut *self.registers }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct KUart;

impl KUart {
    pub fn print_str(&self, str: &str) {
        unsafe { DEVICE.print_str(str.as_bytes()) }
    }
}
