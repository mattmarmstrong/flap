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

    pub fn new(addr: u64) -> VirtualAddress {
        VirtualAddress::try_new(addr).unwrap()
    }

    fn try_new(addr: u64) -> Result<Self, InvalidVirtualAddress> {
        match addr >> 48 {
            0 | 0xFFFF => Ok(VirtualAddress(addr)),
            _ => Err(InvalidVirtualAddress(addr)),
        }
    }

    #[inline]
    pub unsafe fn new_unsafe(addr: u64) -> Self {
        VirtualAddress(addr)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysicalAddress(u64);

#[derive(Debug)]
pub struct InvalidPhysicalAddress(u64);

impl PhysicalAddress {
    #[inline]
    pub const fn zero() -> Self {
        PhysicalAddress(0)
    }

    #[inline]
    pub fn try_new(addr: u64) -> Result<Self, InvalidPhysicalAddress> {
        match addr >> 52 {
            0 => Ok(PhysicalAddress(addr)),
            _ => Err(InvalidPhysicalAddress(addr)),
        }
    }

    #[inline]
    pub unsafe fn new_unsafe(addr: u64) -> Self {
        PhysicalAddress(addr)
    }
}
