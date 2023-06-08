#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtualAddress(pub u64);

#[derive(Debug)]
pub struct InvalidVirtualAddress(u64);

impl VirtualAddress {
    #[inline]
    pub const fn zero() -> Self {
        VirtualAddress(0)
    }

    #[inline]
    pub fn new(address: u64) -> Self {
        VirtualAddress::try_new(address).unwrap()
    }

    #[inline]
    fn try_new(address: u64) -> Result<Self, InvalidVirtualAddress> {
        match address >> 48 {
            0 | 0xFFFF => Ok(VirtualAddress(address)),
            _ => Err(InvalidVirtualAddress(address)),
        }
    }

    #[inline]
    pub fn get_page_offset(self) -> u16 {
        (self.0 & 0x0FFF) as u16
    }

    // we have to use a usize to index into the page table itself, so we might as well do the necessary conversions here
    #[inline]
    pub fn get_level_one_pt_index(self) -> usize {
        ((self.0 >> 12) & 0x01FF) as usize
    }

    #[inline]
    pub fn get_level_two_pt_index(self) -> usize {
        ((self.0 >> 21) & 0x01FF) as usize
    }

    #[inline]
    pub fn get_level_three_pt_index(self) -> usize {
        ((self.0 >> 30) & 0x01FF) as usize
    }

    #[inline]
    pub fn get_level_four_pt_index(self) -> usize {
        ((self.0 >> 39) & 0x01FF) as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysicalAddress(pub u64);

#[derive(Debug)]
pub struct InvalidPhysicalAddress(u64);

impl PhysicalAddress {
    #[inline]
    pub const fn zero() -> Self {
        PhysicalAddress(0)
    }

    #[inline]
    pub fn new(address: u64) -> Self {
        PhysicalAddress::try_new(address).unwrap()
    }

    #[inline]
    fn try_new(address: u64) -> Result<Self, InvalidPhysicalAddress> {
        match address >> 52 {
            0 => Ok(PhysicalAddress(address)),
            _ => Err(InvalidPhysicalAddress(address)),
        }
    }
}

// There was absolutely no need to do this. I just wanted to write a macro
macro_rules! impl_alignment_functions {
    ($addr_type: ty) => {
        impl $addr_type {
            #[inline]
            pub fn align_down(self, alignment: u64) -> Self {
                debug_assert!(alignment.is_power_of_two());
                if self.0 % (alignment) == 0 {
                    self
                } else {
                    let alignment_mask: u64 = !(alignment - 1);
                    let aligned_address: u64 = self.0 & alignment_mask;
                    Self::new(aligned_address)
                }
            }
            #[inline]
            pub fn align_up(self, alignment: u64) -> Self {
                Self::new(self.0 + (alignment - 1)).align_down(alignment)
            }
        }
    };
}

impl_alignment_functions!(VirtualAddress);
impl_alignment_functions!(PhysicalAddress);
