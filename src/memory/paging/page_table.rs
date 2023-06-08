use core::arch::asm;

use crate::memory::address::{PhysicalAddress, VirtualAddress};

const PAGE_TABLE_ENTRIES: usize = 512;

pub fn read_cr3() -> PhysicalAddress {
    let level_4_table_raw_address: u64;
    // bits 0-11 are either the
    let physical_address_mask: u64 = 0x000F_FFFF_FFFF_F000;

    unsafe {
        asm!("mov {}, cr3", out(reg) level_4_table_raw_address, options(nomem, nostack, preserves_flags));
    }
    PhysicalAddress::new(level_4_table_raw_address & physical_address_mask)
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub fn new() -> Self {
        PageTableEntry(0)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(align(4096))]
pub struct PageTable {
    table: [PageTableEntry; PAGE_TABLE_ENTRIES],
}
