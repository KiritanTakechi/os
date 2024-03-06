use core::ops::Add;

use bytemuck::{Pod, Zeroable};

use crate::config::{PAGE_SIZE, PAGE_SIZE_BITS};

const PA_WIDTH_SV39: usize = 56;
const VA_WIDTH_SV39: usize = 39;
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;
const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_SIZE_BITS;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Pod, Zeroable, Debug)]
#[repr(C)]
pub(crate) struct PhysAddr(pub(crate) usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Pod, Zeroable, Debug)]
#[repr(C)]
pub(crate) struct VirtAddr(pub(crate) usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Pod, Zeroable, Debug)]
#[repr(C)]
pub struct PhysPageNum(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Pod, Zeroable, Debug)]
#[repr(C)]
pub struct VirtPageNum(pub usize);

impl From<usize> for PhysAddr {
    fn from(value: usize) -> Self {
        Self(value & ((1 << PA_WIDTH_SV39) - 1))
    }
}

impl From<usize> for PhysPageNum {
    fn from(value: usize) -> Self {
        Self(value & ((1 << PPN_WIDTH_SV39) - 1))
    }
}

impl From<usize> for VirtAddr {
    fn from(value: usize) -> Self {
        Self(value & ((1 << VA_WIDTH_SV39) - 1))
    }
}

impl From<usize> for VirtPageNum {
    fn from(value: usize) -> Self {
        Self(value & ((1 << VPN_WIDTH_SV39) - 1))
    }
}

impl From<PhysAddr> for usize {
    fn from(addr: PhysAddr) -> Self {
        addr.0
    }
}

impl From<PhysPageNum> for usize {
    fn from(addr: PhysPageNum) -> Self {
        addr.0
    }
}

impl From<VirtAddr> for usize {
    fn from(addr: VirtAddr) -> Self {
        if addr.0 >= (1 << (VA_WIDTH_SV39 - 1)) {
            addr.0 | (!((1 << VA_WIDTH_SV39) - 1))
        } else {
            addr.0
        }
    }
}

impl From<VirtPageNum> for usize {
    fn from(addr: VirtPageNum) -> Self {
        addr.0
    }
}

impl From<PhysAddr> for PhysPageNum {
    fn from(addr: PhysAddr) -> Self {
        assert!(addr.is_aligned());
        addr.floor()
    }
}

impl From<PhysPageNum> for PhysAddr {
    fn from(addr: PhysPageNum) -> Self {
        Self(addr.0 << PAGE_SIZE_BITS)
    }
}

impl From<VirtAddr> for VirtPageNum {
    fn from(addr: VirtAddr) -> Self {
        assert!(addr.is_aligned());
        addr.floor()
    }
}

impl From<VirtPageNum> for VirtAddr {
    fn from(addr: VirtPageNum) -> Self {
        Self(addr.0 << PAGE_SIZE_BITS)
    }
}

impl From<VirtAddr> for PhysAddr {
    fn from(addr: VirtAddr) -> Self {
        unimplemented!("VirtAddr -> PhysAddr")
    }
}

impl From<PhysAddr> for VirtAddr {
    fn from(addr: PhysAddr) -> Self {
        unimplemented!("PhysAddr -> VirtAddr")
    }
}

impl Add<usize> for PhysPageNum {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Add<usize> for VirtPageNum {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl PhysAddr {
    pub fn floor(&self) -> PhysPageNum {
        PhysPageNum(self.0 / PAGE_SIZE)
    }
    pub fn ceil(&self) -> PhysPageNum {
        if self.0 == 0 {
            PhysPageNum(0)
        } else {
            PhysPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE)
        }
    }
    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
    pub fn is_aligned(&self) -> bool {
        self.page_offset() == 0
    }
}

impl VirtAddr {
    pub fn floor(&self) -> VirtPageNum {
        VirtPageNum(self.0 / PAGE_SIZE)
    }
    pub fn ceil(&self) -> VirtPageNum {
        if self.0 == 0 {
            VirtPageNum(0)
        } else {
            VirtPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE)
        }
    }
    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
    pub fn is_aligned(&self) -> bool {
        self.page_offset() == 0
    }
}

impl PhysPageNum {}

impl VirtPageNum {
    pub fn page_dir_idxs(&self) -> [usize; 3] {
        let mut virt_page_num = self.0;
        let mut idxs = [0usize; 3];

        (0..3).rev().for_each(|i| {
            idxs[i] = virt_page_num & ((1 << 9) - 1);
            virt_page_num >>= 9;
        });

        idxs
    }
}

pub trait HasPhysAddr {
    fn phys_addr(&self) -> PhysAddr;
}
