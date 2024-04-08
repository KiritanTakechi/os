use core::ops::{Add, Sub};

use bytemuck::{Pod, Zeroable};

use crate::{
    arch::config::{PA_WIDTH, PPN_WIDTH, VA_WIDTH, VPN_WIDTH},
    config::{KERNEL_LOADED_OFFSET_VADDR, PAGE_SIZE, PAGE_SIZE_BITS, PHYS_MEM_BASE_VADDR},
};

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
        Self(value & ((1 << PA_WIDTH) - 1))
    }
}

impl From<usize> for PhysPageNum {
    fn from(value: usize) -> Self {
        Self(value & ((1 << PPN_WIDTH) - 1))
    }
}

impl From<usize> for VirtAddr {
    fn from(value: usize) -> Self {
        Self(value & ((1 << VA_WIDTH) - 1))
    }
}

impl From<usize> for VirtPageNum {
    fn from(value: usize) -> Self {
        Self(value & ((1 << VPN_WIDTH) - 1))
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
        if addr.0 >= (1 << (VA_WIDTH - 1)) {
            addr.0 | (!((1 << VA_WIDTH) - 1))
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
        if (PHYS_MEM_BASE_VADDR.0..=KERNEL_LOADED_OFFSET_VADDR.0).contains(&addr.0) {
            Self(addr.0 - PHYS_MEM_BASE_VADDR.0)
        } else {
            unimplemented!("Page Wark")
        }
    }
}

impl From<PhysAddr> for VirtAddr {
    fn from(addr: PhysAddr) -> Self {
        Self(addr.0 + PHYS_MEM_BASE_VADDR.0)
    }
}

impl PhysAddr {
    pub(crate) fn floor(&self) -> PhysPageNum {
        PhysPageNum(self.0 >> PAGE_SIZE_BITS)
    }
    pub(crate) fn ceil(&self) -> PhysPageNum {
        if self.0 == 0 {
            PhysPageNum(0)
        } else {
            PhysPageNum((self.0 - 1 + PAGE_SIZE) >> PAGE_SIZE_BITS)
        }
    }
    pub(crate) fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
    pub(crate) fn is_aligned(&self) -> bool {
        self.page_offset() == 0
    }
}

impl VirtAddr {
    pub(crate) fn floor(&self) -> VirtPageNum {
        VirtPageNum(self.0 >> PAGE_SIZE_BITS)
    }
    pub(crate) fn ceil(&self) -> VirtPageNum {
        if self.0 == 0 {
            VirtPageNum(0)
        } else {
            VirtPageNum((self.0 - 1 + PAGE_SIZE) >> PAGE_SIZE_BITS)
        }
    }
    pub(crate) fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
    pub(crate) fn is_aligned(&self) -> bool {
        self.page_offset() == 0
    }
}

impl Add<usize> for PhysAddr {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Add<usize> for VirtAddr {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
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

impl Add<PhysAddr> for PhysAddr {
    type Output = Self;

    fn add(self, rhs: PhysAddr) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<VirtAddr> for VirtAddr {
    type Output = Self;

    fn add(self, rhs: VirtAddr) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<PhysPageNum> for PhysPageNum {
    type Output = Self;

    fn add(self, rhs: PhysPageNum) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<VirtPageNum> for VirtPageNum {
    type Output = Self;

    fn add(self, rhs: VirtPageNum) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub<usize> for PhysAddr {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub<usize> for VirtAddr {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub<usize> for PhysPageNum {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub<usize> for VirtPageNum {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub<PhysAddr> for PhysAddr {
    type Output = Self;

    fn sub(self, rhs: PhysAddr) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Sub<VirtAddr> for VirtAddr {
    type Output = Self;

    fn sub(self, rhs: VirtAddr) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Sub<PhysPageNum> for PhysPageNum {
    type Output = Self;

    fn sub(self, rhs: PhysPageNum) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Sub<VirtPageNum> for VirtPageNum {
    type Output = Self;

    fn sub(self, rhs: VirtPageNum) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
