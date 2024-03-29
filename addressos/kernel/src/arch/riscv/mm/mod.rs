use core::arch::asm;

use bitflags::bitflags;
use bytemuck::{Pod, Zeroable};
use riscv::register::satp;

use crate::{
    config::ENTRY_COUNT,
    mm::{
        address::{PhysAddr, PhysPageNum, VirtAddr},
        page_table::{PageTableEntryTrait, PageTableFlagsTrait},
    },
};

bitflags! {
    #[derive(Clone, Copy, Pod, Zeroable, Debug)]
    #[repr(C)]
    pub(crate) struct PageTableFlags: u8 {
        /// 使能位
        const Valid = 1 << 0;
        /// 可读
        const Read = 1 << 1;
        /// 可写
        const Write = 1 << 2;
        /// 可执行
        const Execute = 1 << 3;
        /// 可用户模式下访问
        const User = 1 << 4;
        /// 全局位
        const Global = 1 << 5;
        /// 访问记录
        const Accessed = 1 << 6;
        /// 修改记录
        const Dirty = 1 << 7;
    }
}

impl PageTableFlagsTrait for PageTableFlags {
    fn new() -> Self {
        Self::empty()
    }

    fn set_valid(&mut self, valid: bool) -> Self {
        self.set(Self::Valid, valid);
        *self
    }

    fn set_writable(&mut self, writable: bool) -> Self {
        self.set(Self::Write, writable);
        *self
    }

    fn set_readable(&mut self, readable: bool) -> Self {
        self.set(Self::Read, readable);
        *self
    }

    fn set_accessible_by_user(&mut self, accessible: bool) -> Self {
        self.set(Self::User, accessible);
        *self
    }

    fn set_executable(&mut self, executable: bool) -> Self {
        self.set(Self::Execute, executable);
        *self
    }

    fn is_valid(&self) -> bool {
        self.contains(Self::Valid)
    }

    fn is_writable(&self) -> bool {
        self.contains(Self::Write)
    }

    fn is_readable(&self) -> bool {
        self.contains(Self::Read)
    }

    fn is_accessible_by_user(&self) -> bool {
        self.contains(Self::User)
    }

    fn is_executable(&self) -> bool {
        self.contains(Self::Execute)
    }

    fn is_accessed(&self) -> bool {
        self.contains(Self::Accessed)
    }

    fn is_dirty(&self) -> bool {
        self.contains(Self::Dirty)
    }
}

pub fn tlb_flush(addr: VirtAddr) {
    unsafe {
        asm!("sfence.vma {}", in(reg) usize::from(addr), options(nostack));
    }
}

pub fn mm_csr(root_addr:PhysAddr){
    let stap_bit = 8usize << 60 | root_addr.floor().0;
    unsafe {
        satp::write(stap_bit);
        asm!("sfence.vma");
    }
}

#[derive(Clone, Copy, Pod, Zeroable, Debug)]
#[repr(C)]
pub struct PageTableEntry(usize);

impl PageTableEntryTrait for PageTableEntry {
    type F = PageTableFlags;

    fn page_index(addr: VirtAddr, level: usize) -> usize {
        debug_assert!((1..=5).contains(&level));
        usize::from(addr) >> (12 + 9 * (level - 1)) & (ENTRY_COUNT - 1)
    }

    fn phys_page_num(&self) -> PhysPageNum {
        PhysPageNum(self.0 >> 10)
    }

    fn flags(&self) -> Self::F {
        PageTableFlags::from_bits_truncate(self.0 as u8)
    }

    fn is_used(&self) -> bool {
        self.0 != 0
    }

    fn update(&mut self, phys_page_num: PhysPageNum, flags: Self::F) {
        self.0 = usize::from(phys_page_num) << 10 | flags.bits() as usize;
    }

    fn clear(&mut self) {
        self.0 = 0;
    }
}

