use core::{marker::PhantomData, ptr};

use alloc::sync::Arc;
use bytemuck::{bytes_of, Pod, Zeroable};

use crate::config::PAGE_SIZE;

use super::{address::{PhysAddr, PhysPageNum}, frame_allocator};

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

impl Drop for VirtMemFrame {
    fn drop(&mut self) {
        if Arc::strong_count(&self.frame_index) == 1 {
            frame_allocator::dealloc(self.frame_index());
        }
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

    pub(crate) fn as_ptr(&self) -> *const usize {
        let addr = self.start_phys_addr();
        ptr::from_ref(&addr.0)
    }

    pub(crate) fn as_mut_ptr(&self) -> *mut usize {
        let mut addr = self.start_phys_addr();
        ptr::from_mut(&mut addr.0)
    }

    pub fn copy_from_frame(&self, src: &Self) {
        if Arc::ptr_eq(&self.frame_index, &src.frame_index) {
            return;
        }

        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(), self.as_mut_ptr(), PAGE_SIZE);
        }
    }
}

pub struct VirtMemReader<'a> {
    cursor: *const u8,
    end: *const u8,
    phantom: PhantomData<&'a [u8]>,
}

impl<'a> VirtMemReader<'a> {
    pub(crate) const unsafe fn from_raw_parts(ptr: *const u8, len: usize) -> Self {
        Self {
            cursor: ptr,
            end: ptr.add(len),
            phantom: PhantomData,
        }
    }

    pub(crate) const fn remain(&self) -> usize {
        unsafe { self.end.sub_ptr(self.cursor) }
    }

    pub(crate) const fn cursor(&self) -> *const u8 {
        self.cursor
    }

    pub(crate) const fn has_remain(&self) -> bool {
        self.remain() > 0
    }

    pub(crate) const fn limit(mut self, max_remain: usize) -> Self {
        if max_remain < self.remain() {
            unsafe { self.end = self.cursor.add(max_remain) };
        }
        self
    }
    pub(crate) fn skip(mut self, nbytes: usize) -> Self {
        assert!(nbytes <= self.remain());

        unsafe { self.cursor = self.cursor.add(nbytes) };
        self
    }

    pub(crate) fn read(&mut self, writer: &mut VirtMemWriter<'_>) -> usize {
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
    pub(crate) const unsafe fn from_raw_parts_mut(ptr: *mut u8, len: usize) -> Self {
        Self {
            cursor: ptr,
            end: ptr.add(len),
            phantom: PhantomData,
        }
    }

    pub(crate) const fn avail(&self) -> usize {
        unsafe { self.end.sub_ptr(self.cursor) }
    }

    pub(crate) const fn cursor(&self) -> *mut u8 {
        self.cursor
    }

    pub(crate) const fn has_avail(&self) -> bool {
        self.avail() > 0
    }

    pub(crate) const fn limit(mut self, max_avail: usize) -> Self {
        if max_avail < self.avail() {
            unsafe { self.end = self.cursor.add(max_avail) };
        }
        self
    }

    pub(crate) fn skip(mut self, nbytes: usize) -> Self {
        assert!(nbytes <= self.avail());

        unsafe { self.cursor = self.cursor.add(nbytes) };
        self
    }

    pub(crate) fn write(&mut self, reader: &mut VirtMemReader<'_>) -> usize {
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

    pub(crate) fn fill<T: Pod + Zeroable>(&mut self, value: T) {
        assert!(self.avail() / bytes_of(&value).len() > 0);
        assert!(self.avail() % bytes_of(&value).len() == 0);

        while self.avail() > 0 {
            self.write(&mut bytes_of(&value).into());
        }
    }
}

impl<'a> From<&'a mut [u8]> for VirtMemWriter<'a> {
    fn from(slice: &'a mut [u8]) -> Self {
        unsafe { Self::from_raw_parts_mut(slice.as_mut_ptr(), slice.len()) }
    }
}
