use core::marker::PhantomData;

use alloc::vec::Vec;

use super::{address::PhysAddr, frame::VirtMemFrame};

pub(crate) trait PageTableFlagsTrait: Clone + Copy + Sized {
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

pub(crate) trait PageTableEntryTrait: Clone + Copy + Sized {
    type F: PageTableFlagsTrait;
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
}
