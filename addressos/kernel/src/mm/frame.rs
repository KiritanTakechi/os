use crate::config::PAGE_SIZE;

use super::address::{HasPhysAddr, PhysAddr, PhysPageNum};

pub(crate) struct VirtMemFrame {
    pub(crate) frame_index: PhysPageNum,
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
    fn frame_index(&self) -> PhysPageNum {
        self.frame_index
    }

    pub(crate) fn start_phys_addr(&self) -> PhysAddr {
        todo!()
    }
}