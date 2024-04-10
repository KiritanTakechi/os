use crate::mm::address::VirtAddr;

pub(crate) const KERNEL_HEAP_SIZE: usize = 0x30_0000;

pub(crate) const PAGE_SIZE: usize = 0x1000;
pub(crate) const PAGE_SIZE_BITS: usize = 0xc;
pub(crate) const ENTRY_COUNT: usize = 512;

pub(crate) const KERNEL_OFFSET: usize = 0xffff_ffff_8000_0000;

pub(crate) const PHYS_MEM_BASE_VADDR: VirtAddr = VirtAddr(0xffff_8000_0000_0000);
pub(crate) const KERNEL_LOADED_OFFSET_VADDR: VirtAddr = VirtAddr(0xffff_ffff_8000_0000);

pub(crate) const MEMORY_END: usize = 0x8800_0000;
