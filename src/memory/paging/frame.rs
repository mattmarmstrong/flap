use crate::memory::address::PhysicalAddress;
use crate::memory::paging::consts::PageSize;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalFrame {
    size: PageSize,
    offset: PhysicalAddress,
}

impl PhysicalFrame {
    // yields the frame that contains the physical address
    pub fn from_address_aligned(physical_address: PhysicalAddress, page_size: PageSize) -> Self {
        let aligned_address = physical_address.align_down(page_size as u64);
        PhysicalFrame {
            size: page_size,
            offset: aligned_address,
        }
    }
}
