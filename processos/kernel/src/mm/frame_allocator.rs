use buddy_system_allocator::FrameAllocator;
use spin::{Mutex, Once};

use crate::{config::MEMORY_END, ffi::ekernel};

use super::{
    address::{PhysAddr, PhysPageNum},
    frame::VirtMemFrame,
};

pub(super) static FRAME_ALLOCATOR: Once<Mutex<FrameAllocator>> = Once::new();

pub(crate) fn alloc() -> Option<VirtMemFrame> {
    FRAME_ALLOCATOR
        .get()
        .unwrap()
        .lock()
        .alloc(1)
        .map(|start| VirtMemFrame::new(start.into()))
}

pub(crate) fn dealloc(frame_index: PhysPageNum) {
    FRAME_ALLOCATOR
        .get()
        .unwrap()
        .lock()
        .dealloc(frame_index.into(), 1);
}

pub fn init_frame_allocator() {
    let mut allocator = FrameAllocator::<32>::new();

    allocator.add_frame(
        PhysAddr::from(ekernel as usize).ceil().0,
        PhysAddr::from(MEMORY_END).floor().0,
    );

    FRAME_ALLOCATOR.call_once(|| Mutex::new(allocator));
}
