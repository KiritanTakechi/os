use alloc::vec::Vec;

use crate::error::Error;

use super::{frame::VirtMemFrame, frame_allocator};

pub struct VirtMemAllocOption {
    frame_num: usize,
    is_uninit: bool,
}

impl VirtMemAllocOption {
    pub fn new(frame_num: usize) -> Self {
        Self {
            frame_num,
            is_uninit: false,
        }
    }

    pub fn set_uninit(&mut self, uninit: bool) -> &mut Self {
        self.is_uninit = uninit;
        self
    }

    pub fn alloc(&self) -> Result<Vec<VirtMemFrame>, Error> {
        let frames = {
            let mut frame_list = Vec::new();
            for _ in 0..self.frame_num {
                frame_list.push(frame_allocator::alloc().ok_or(Error::NoMemory)?);
            }
            frame_list
        };
        if !self.is_uninit {
            for frame in frames.iter() {
                frame.writer().fill(0);
            }
        }

        Ok(frames)
    }

    pub fn alloc_single(&self) -> Result<VirtMemFrame, Error> {
        if self.frame_num != 1 {
            return Err(Error::InvalidArgs);
        }

        let frame = frame_allocator::alloc().ok_or(Error::NoMemory)?;

        if !self.is_uninit {
            frame.writer().fill(0);
        }

        Ok(frame)
    }
}

#[allow(unused)]
/// a simple test for frame allocator
pub fn frame_allocator_test() {
    let mut v: Vec<VirtMemFrame> = Vec::new();
    for i in 0..5 {
        let frame = VirtMemAllocOption::new(1).alloc_single().unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    v.clear();
    for i in 0..5 {
        let frame = VirtMemAllocOption::new(1).alloc_single().unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    drop(v);
    println!("frame_allocator_test passed!");
}
