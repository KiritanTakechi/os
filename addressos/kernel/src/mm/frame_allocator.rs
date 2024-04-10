use crate::{config::MEMORY_END, mm::address::PhysAddr};

use super::{address::PhysPageNum, frame::VirtMemFrame};

use alloc::vec::Vec;
use buddy_system_allocator::LockedFrameAllocator;
use spin::Once;

pub(super) static FRAME_ALLOCATOR: Once<LockedFrameAllocator> = Once::new();

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
    extern "C" {
        fn ekernel();
    }
    println!("ekernel: 0x{:x?}", ekernel as usize);

    let allocator = LockedFrameAllocator::<32>::new();

    allocator.lock().add_frame(
        PhysAddr::from(ekernel as usize).ceil().0,
        PhysAddr::from(MEMORY_END).floor().0,
    );

    FRAME_ALLOCATOR.call_once(|| allocator);
}

#[allow(unused)]
/// a simple test for frame allocator
pub fn frame_allocator_test() {
    let mut v: Vec<VirtMemFrame> = Vec::new();
    for i in 0..5 {
        let frame = alloc().unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    v.clear();
    for i in 0..5 {
        let frame = alloc().unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    drop(v);
    println!("frame_allocator_test passed!");
}
