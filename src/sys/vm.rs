use bitfield::bitfield;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_arch="aarch64")] {
        bitfield! {
            struct TableDescriptor(u64);
            address, set_address: 47, 12;
            attributes, set_attributes: 63, 59;
        }

        bitfield! {
            struct PageDescriptor(u64);
            lower_attributes, set_lower_attributes: 11, 2;
            address, set_address: 47, 12;
            upper_attributes, set_upper_attributes: 63, 59;
        }
    }
}

#[link_section = ".bss.ttable"]
static L0_TABLE: [u64; 16] = [0; 16];
#[link_section = ".bss.ttable"]
static L1_TABLE: [u64; 16] = [0; 16];
#[link_section = ".bss.ttable"]
static L2_TABLE: [u64; 16] = [0; 16];
#[link_section = ".bss.ttable"]
static L3_TABLE: [u64; 16] = [0; 16];

pub unsafe fn init() {}
