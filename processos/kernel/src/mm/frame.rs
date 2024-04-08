use core::ptr;

use alloc::sync::Arc;

use crate::config::PAGE_SIZE;

use super::address::{PhysAddr, PhysPageNum};

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

// impl Drop for VirtMemFrame {
//     fn drop(&mut self) {
//         if Arc::strong_count(&self.frame_index) == 1 {
//             frame_allocator::dealloc(self.frame_index());
//         }
//     }
// }

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
