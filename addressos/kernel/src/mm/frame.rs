use core::marker::PhantomData;

use alloc::{sync::Arc, vec::Vec};
use pod::Pod;

use crate::config::PAGE_SIZE;

use super::{
    address::{HasPhysAddr, PhysAddr, PhysPageNum},
    frame_allocator,
};

#[derive(Debug)]
pub(crate) struct VirtMemFrame {
    frame_index: Arc<PhysPageNum>,
}

impl Clone for VirtMemFrame {
    fn clone(&self) -> Self {
        VirtMemFrame {
            frame_index: self.frame_index.clone(),
        }
    }
}

impl HasPhysAddr for VirtMemFrame {
    fn phys_addr(&self) -> PhysAddr {
        self.start_phys_addr()
    }
}

impl VirtMemFrame {
    pub(crate) fn new(frame_index: PhysPageNum) -> Self {
        VirtMemFrame {
            frame_index: frame_index.into(),
        }
    }

    pub(crate) fn frame_index(&self) -> PhysPageNum {
        *self.frame_index
    }

    pub(crate) fn start_phys_addr(&self) -> PhysAddr {
        (*self.frame_index).into()
    }

    pub(crate) fn end_phys_addr(&self) -> PhysAddr {
        (*self.frame_index + 1).into()
    }

    pub(crate) fn as_ptr(&self) -> *const u8 {
        let addr = self.start_phys_addr();
        addr.0 as *const u8
    }

    pub(crate) fn as_mut_ptr(&self) -> *mut u8 {
        let addr = self.start_phys_addr();
        addr.0 as *mut u8
    }
}

impl<'a> VirtMemFrame {
    pub fn reader(&'a self) -> VirtMemReader<'a> {
        unsafe { VirtMemReader::from_raw_parts(self.as_ptr(), PAGE_SIZE) }
    }

    pub fn writer(&'a self) -> VirtMemWriter<'a> {
        unsafe { VirtMemWriter::from_raw_parts_mut(self.as_mut_ptr(), PAGE_SIZE) }
    }
}

impl Drop for VirtMemFrame {
    fn drop(&mut self) {
        if Arc::strong_count(&self.frame_index) == 1 {
            frame_allocator::dealloc(self.frame_index());
        }
    }
}

pub struct VirtMemReader<'a> {
    cursor: *const u8,
    end: *const u8,
    phantom: PhantomData<&'a [u8]>,
}

impl<'a> VirtMemReader<'a> {
    pub const unsafe fn from_raw_parts(ptr: *const u8, len: usize) -> Self {
        Self {
            cursor: ptr,
            end: ptr.add(len),
            phantom: PhantomData,
        }
    }

    pub const fn remain(&self) -> usize {
        unsafe { self.end.sub_ptr(self.cursor) }
    }

    pub const fn cursor(&self) -> *const u8 {
        self.cursor
    }

    pub const fn has_remain(&self) -> bool {
        self.remain() > 0
    }

    pub const fn limit(mut self, max_remain: usize) -> Self {
        if max_remain < self.remain() {
            unsafe { self.end = self.cursor.add(max_remain) };
        }
        self
    }
    pub fn skip(mut self, nbytes: usize) -> Self {
        assert!(nbytes <= self.remain());

        unsafe { self.cursor = self.cursor.add(nbytes) };
        self
    }

    pub fn read(&mut self, writer: &mut VirtMemWriter<'_>) -> usize {
        let copy_len = self.remain().min(writer.avail());
        if copy_len == 0 {
            return 0;
        }

        unsafe {
            core::ptr::copy(self.cursor, writer.cursor, copy_len);
            self.cursor = self.cursor.add(copy_len);
            writer.cursor = writer.cursor.add(copy_len);
        }
        copy_len
    }
}

impl<'a> From<&'a [u8]> for VirtMemReader<'a> {
    fn from(slice: &'a [u8]) -> Self {
        unsafe { Self::from_raw_parts(slice.as_ptr(), slice.len()) }
    }
}

pub struct VirtMemWriter<'a> {
    cursor: *mut u8,
    end: *mut u8,
    phantom: PhantomData<&'a mut [u8]>,
}

impl<'a> VirtMemWriter<'a> {
    pub const unsafe fn from_raw_parts_mut(ptr: *mut u8, len: usize) -> Self {
        Self {
            cursor: ptr,
            end: ptr.add(len),
            phantom: PhantomData,
        }
    }

    pub const fn avail(&self) -> usize {
        unsafe { self.end.sub_ptr(self.cursor) }
    }

    pub const fn cursor(&self) -> *mut u8 {
        self.cursor
    }

    pub const fn has_avail(&self) -> bool {
        self.avail() > 0
    }

    pub const fn limit(mut self, max_avail: usize) -> Self {
        if max_avail < self.avail() {
            unsafe { self.end = self.cursor.add(max_avail) };
        }
        self
    }

    pub fn skip(mut self, nbytes: usize) -> Self {
        assert!(nbytes <= self.avail());

        unsafe { self.cursor = self.cursor.add(nbytes) };
        self
    }

    pub fn write(&mut self, reader: &mut VirtMemReader<'_>) -> usize {
        let copy_len = self.avail().min(reader.remain());
        if copy_len == 0 {
            return 0;
        }

        unsafe {
            core::ptr::copy(reader.cursor, self.cursor, copy_len);
            self.cursor = self.cursor.add(copy_len);
            reader.cursor = reader.cursor.add(copy_len);
        }
        copy_len
    }

    pub fn fill<T: Pod>(&mut self, value: T) {
        assert!(self.avail() / value.as_bytes().len() > 0);
        assert!(self.avail() % value.as_bytes().len() == 0);

        while self.avail() > 0 {
            self.write(&mut value.as_bytes().into());
        }
    }
}

impl<'a> From<&'a mut [u8]> for VirtMemWriter<'a> {
    fn from(slice: &'a mut [u8]) -> Self {
        unsafe { Self::from_raw_parts_mut(slice.as_mut_ptr(), slice.len()) }
    }
}
