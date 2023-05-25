use core::arch::asm;
use core::mem::size_of;

use lazy_static::lazy_static;

use crate::memory::address::VirtualAddress;
use crate::structs::tss::{TaskStateSegment, TSS};

#[derive(Debug, Clone, Copy)]
pub struct GlobalDescriptorTable {
    descriptor_table: [SegmentDescriptor; 7],
}

#[repr(C, packed(2))]
pub struct GdtPointer {
    limit: u16,
    base: VirtualAddress,
}

impl GlobalDescriptorTable {
    fn new() -> GlobalDescriptorTable {
        GlobalDescriptorTable {
            descriptor_table: [SegmentDescriptor::null_segment_descriptor(); 7],
        }
    }

    fn add_entry(&mut self, index: usize, segment_desc: SegmentDescriptor) -> SegmentSelector {
        self.descriptor_table[index] = segment_desc;
        SegmentSelector::new(index as u16, segment_desc.get_requested_privilege_level())
    }

    fn pointer(&self) -> GdtPointer {
        let limit = (self.descriptor_table.len() * size_of::<SegmentDescriptor>() - 1) as u16;
        let base = VirtualAddress::new(self.descriptor_table.as_ptr() as u64);
        GdtPointer { limit, base }
    }

    pub unsafe fn load_gdt(gdt_ptr: &GdtPointer) {
        asm!("lgdt [{}]", in(reg) gdt_ptr, options(readonly, nostack, preserves_flags))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct SegmentDescriptor {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl SegmentDescriptor {
    // See the section on Segment Descriptors here -> https://wiki.osdev.org/Global_Descriptor_Table
    const fn null_segment_descriptor() -> SegmentDescriptor {
        SegmentDescriptor {
            limit_low: 0x0000,
            base_low: 0x0000,
            base_middle: 0x00,
            access: 0x00,
            granularity: 0x00,
            base_high: 0x00,
        }
    }

    const fn kernel_code_segment_descriptor() -> Self {
        SegmentDescriptor {
            limit_low: 0xFFFF,
            base_low: 0x0000,
            base_middle: 0x00,
            access: 0x9A,
            granularity: 0xAF,
            base_high: 0x00,
        }
    }

    const fn kernel_data_segment_descriptor() -> Self {
        SegmentDescriptor {
            limit_low: 0xFFFF,
            base_low: 0x0000,
            base_middle: 0x00,
            access: 0x92,
            granularity: 0xAF,
            base_high: 0x00,
        }
    }

    const fn user_code_segment_descriptor() -> Self {
        SegmentDescriptor {
            limit_low: 0xFFFF,
            base_low: 0x0000,
            base_middle: 0x00,
            access: 0xFA,
            granularity: 0xAF,
            base_high: 0x00,
        }
    }

    const fn user_data_segment_descriptor() -> Self {
        SegmentDescriptor {
            limit_low: 0xFFFF,
            base_low: 0x0000,
            base_middle: 0x00,
            access: 0xF2,
            granularity: 0xAF,
            base_high: 0x00,
        }
    }

    fn tss_system_segment(tss: &'static TaskStateSegment) -> (Self, Self) {
        let tss_ptr = tss.pointer();
        let tss_system_segment_low = SegmentDescriptor {
            limit_low: (size_of::<TaskStateSegment>() - 1) as u16,
            base_low: (tss_ptr.0 & 0x0000_FFFF) as u16,
            base_middle: ((tss_ptr.0 & 0x00FF_0000) >> 16) as u8,
            access: 0xE9_u8,
            granularity: 0x00_u8,
            base_high: ((tss_ptr.0 & 0xFF00_0000) >> 24) as u8,
        };
        let tss_system_segment_high = SegmentDescriptor {
            limit_low: ((tss_ptr.0 & 0x0000_FFFF_0000_0000) >> 32) as u16,
            base_low: ((tss_ptr.0 & 0xFFFF_0000_0000_0000) >> 48) as u16,
            base_middle: 0,
            access: 0,
            granularity: 0,
            base_high: 0,
        };
        (tss_system_segment_low, tss_system_segment_high)
    }

    // convenience method
    pub fn get_requested_privilege_level(&self) -> u8 {
        let rpl_bit_mask: u8 = 0b0110_0000;
        self.access & rpl_bit_mask
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct SegmentSelector(pub u16);

impl SegmentSelector {
    pub fn new(index: u16, dpl: u8) -> SegmentSelector {
        SegmentSelector(index << 3 | dpl as u16)
    }
}

pub struct Selectors {
    pub kernel_code_segment: SegmentSelector,
    pub kernel_data_segment: SegmentSelector,
    pub user_code_segment: SegmentSelector,
    pub user_data_segment: SegmentSelector,
    pub tss_system_segment: SegmentSelector,
}

lazy_static! {
    pub static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        // the null segment descriptor is left alone as the 0th entry of the GDT
        let kernel_code_segment =
            gdt.add_entry(1, SegmentDescriptor::kernel_code_segment_descriptor());
        let kernel_data_segment =
            gdt.add_entry(2, SegmentDescriptor::kernel_data_segment_descriptor());
        let user_code_segment = gdt.add_entry(3, SegmentDescriptor::user_code_segment_descriptor());
        let user_data_segment = gdt.add_entry(4, SegmentDescriptor::user_data_segment_descriptor());
        let (tss_system_segment_low, tss_system_segment_high) =
            SegmentDescriptor::tss_system_segment(&TSS);
        let tss_system_segment = gdt.add_entry(5, tss_system_segment_low);
        let _ = gdt.add_entry(6, tss_system_segment_high); // we shouldn't ever need to reference the high segment of a system segment
        let selectors = Selectors {
            kernel_code_segment,
            kernel_data_segment,
            user_code_segment,
            user_data_segment,
            tss_system_segment,
        };
        (gdt, selectors)
    };
}

pub fn init_gdt() {
    unsafe {
        GlobalDescriptorTable::load_gdt(&GDT.0.pointer());
    }
}
