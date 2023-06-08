use crate::memory::address::VirtualAddress;
use crate::memory::paging::consts::PageSize;

#[repr(C)]
pub struct Page {
    size: PageSize,
    offset: VirtualAddress,
}

impl Page {
    pub fn from_address_aligned(virtual_address: VirtualAddress, page_size: PageSize) -> Self {
        let aligned_address = virtual_address.align_down(page_size as u64);
        Page {
            size: page_size,
            offset: aligned_address,
        }
    }
}
