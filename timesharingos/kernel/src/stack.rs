use core::{mem::size_of, ptr::copy};

use crate::{
    config::{KERNEL_STACK_SIZE, MAX_APP_NUM, USER_STACK_SIZE},
    trap::context::TrapContext,
};

#[derive(Copy, Clone)]
#[repr(align(4096))]
pub(crate) struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

pub(crate) static KERNEL_STACK: [KernelStack; MAX_APP_NUM] = [KernelStack {
    data: [0; KERNEL_STACK_SIZE],
}; MAX_APP_NUM];

impl KernelStack {
    pub(crate) fn get_top_ptr(&self) -> usize {
        self.data.as_ptr().wrapping_add(KERNEL_STACK_SIZE) as usize
    }

    pub(crate) fn push_context(&self, context: TrapContext) -> usize {
        let context_size = size_of::<TrapContext>();
        let context_ptr = self.get_top_ptr().wrapping_sub(context_size) as *mut u8;
        unsafe {
            copy(
                &context as *const TrapContext as *const u8,
                context_ptr,
                context_size,
            );
        }
        context_ptr as usize
    }
}

#[derive(Copy, Clone)]
#[repr(align(4096))]
pub(crate) struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

pub(crate) static USER_STACK: [UserStack; MAX_APP_NUM] = [UserStack {
    data: [0; USER_STACK_SIZE],
}; MAX_APP_NUM];

impl UserStack {
    pub(crate) fn top_ptr(&self) -> usize {
        self.data.as_ptr().wrapping_add(USER_STACK_SIZE) as usize
    }
}
