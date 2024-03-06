use core::marker::PhantomData;

use alloc::vec::Vec;
use bytemuck::{Pod, Zeroable};

use crate::mm::frame_allocator;

use super::{
    address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum},
    frame::VirtMemFrame,
};

pub(crate) trait PageTableFlagsTrait: Clone + Copy + Sized + Pod + Zeroable {
    fn new() -> Self;

    fn set_valid(&mut self, valid: bool) -> Self;

    fn set_writable(&mut self, writable: bool) -> Self;

    fn set_readable(&mut self, readable: bool) -> Self;

    fn set_accessible_by_user(&mut self, accessible: bool) -> Self;

    fn set_executable(&mut self, executable: bool) -> Self;

    fn is_valid(&self) -> bool;

    fn is_writable(&self) -> bool;

    fn is_readable(&self) -> bool;

    fn is_accessible_by_user(&self) -> bool;

    fn is_executable(&self) -> bool;

    fn is_accessed(&self) -> bool;

    fn is_dirty(&self) -> bool;
}

pub(crate) trait PageTableEntryTrait: Clone + Copy + Sized + Pod + Zeroable {
    type F: PageTableFlagsTrait;

    fn phys_page_num(&self) -> PhysPageNum;

    fn flags(&self) -> Self::F;

    fn update(&mut self, phys_page_num: PhysPageNum, flags: Self::F);

    fn clear(&mut self);
}

#[derive(Debug)]
pub(crate) enum PageTableError {
    InvalidModification,
    InvalidVaddr,
}

#[derive(Clone)]
pub(crate) struct UserMode;

#[derive(Clone)]
pub(crate) struct KernelMode;

#[derive(Clone)]
pub(crate) struct DeviceMode;

pub(crate) struct PageTable<T: PageTableEntryTrait> {
    root_paddr: PhysAddr,
    tables: Vec<VirtMemFrame>,
    phantom: PhantomData<T>,
}

impl<T: PageTableEntryTrait> PageTable<T> {
    fn new(root_paddr: PhysAddr) -> Self {
        Self {
            root_paddr,
            tables: Vec::new(),
            phantom: PhantomData,
        }
    }

    fn page_walk(&mut self, virt_page_num: VirtPageNum, create: bool) -> Option<&mut T> {
        let idxs = virt_page_num.page_dir_idxs();
        let mut page_table = self.root_paddr;
        let mut page_table_entry = None;

        for i in 0..3 {
            let idx = idxs[i];
            let page_table_entry_paddr = page_table + idx * core::mem::size_of::<T>();
            let page_table_entry_vaddr = VirtAddr::from(page_table_entry_paddr);
            let page_table_entry = unsafe { &mut *(page_table_entry_vaddr.0 as *mut T) };

            if page_table_entry.flags().is_valid() {
                page_table = page_table_entry.phys_page_num().into();
            } else if create {
                let frame = frame_allocator::alloc().unwrap();
                page_table_entry.update(PhysPageNum::from(frame.start_phys_addr()), T::F::new());
                page_table = frame.start_phys_addr();
                self.tables.push(frame);
            } else {
                return None;
            }
        }

        page_table_entry
    }
}
