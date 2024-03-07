use core::{marker::PhantomData, mem::size_of};

use alloc::vec::Vec;
use bytemuck::{Pod, Zeroable};

use crate::{arch::mm::tlb_flush, config::PAGE_SIZE, mm::frame_allocator};

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

    fn page_index(addr: VirtAddr, level: usize) -> usize;

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

    fn page_walk(&mut self, addr: VirtAddr, create: bool) -> Option<&mut T> {
        let mut count = 3;

        let mut current_entry = unsafe {
            &mut *((usize::from(self.root_paddr) + size_of::<T>() * T::page_index(addr, count))
                as *mut T)
        };

        while count > 1 {
            if !current_entry.flags().is_valid() {
                if !create {
                    return None;
                }

                let frame = frame_allocator::alloc().unwrap();

                let flags = T::F::new()
                    .set_valid(true)
                    .set_accessible_by_user(true)
                    .set_readable(true)
                    .set_writable(true);
                current_entry.update(frame.start_phys_addr().into(), flags);
                self.tables.push(frame);
            }

            // if current_entry.flags().is_huge() {
            //     break;
            // }

            count -= 1;
            debug_assert!(size_of::<T>() * (T::page_index(addr, count) + 1) <= PAGE_SIZE);

            current_entry = unsafe {
                &mut *((usize::from(self.root_paddr) + size_of::<T>() * T::page_index(addr, count))
                    as *mut T)
            };
        }
        Some(current_entry)
    }

    fn map(&mut self, addr: VirtAddr, target: PhysAddr, flags: T::F) -> Result<(), PageTableError> {
        let entry = self
            .page_walk(addr, true)
            .ok_or(PageTableError::InvalidVaddr)?;

        if entry.flags().is_valid() {
            return Err(PageTableError::InvalidModification);
        }

        entry.update(target.floor(), flags);
        Ok(())
    }

    fn unmap(&mut self, addr: VirtAddr) -> Result<(), PageTableError> {
        let entry = self
            .page_walk(addr, false)
            .ok_or(PageTableError::InvalidVaddr)?;

        if !entry.flags().is_valid() {
            return Err(PageTableError::InvalidModification);
        }

        entry.clear();
        tlb_flush(addr);
        Ok(())
    }
}
