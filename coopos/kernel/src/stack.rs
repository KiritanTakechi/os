use core::{mem::size_of, ptr::copy};

use crate::{config::{KERNEL_STACK_SIZE, MAX_APP_NUM, USER_STACK_SIZE}, trap::context::TrapContext};

#[derive(Copy, Clone)]
#[repr(align(4096))]
pub(crate) struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
    sp: usize,
}

pub(crate) static KERNEL_STACK: [KernelStack; MAX_APP_NUM] = [KernelStack {
    data: [0; KERNEL_STACK_SIZE],
    sp: KERNEL_STACK_SIZE,
}; MAX_APP_NUM];

impl KernelStack {
    pub(crate) fn top_ptr(&self) -> *const u8 {
        self.data.as_ptr().wrapping_add(KERNEL_STACK_SIZE)
    }

    pub(crate) fn sp_ptr(&self) -> *const u8 {
        self.data[self.sp..].as_ptr()
    }

    pub(crate) fn push_context(&mut self, context: &TrapContext) {
        let context_size = size_of::<TrapContext>();
        assert!(self.sp >= context_size, "Stack overflow");

        self.sp -= context_size;
        let context_ptr = context as *const _ as *const u8;
        let stack_ptr = unsafe { self.data.as_mut_ptr().add(self.sp) };
        unsafe {
            copy(context_ptr, stack_ptr, context_size);
        }
    }
}

#[derive(Copy, Clone)]
#[repr(align(4096))]
pub(crate) struct UserStack {
    data: [u8; USER_STACK_SIZE],
    sp: usize,
}

pub(crate) static USER_STACK: [UserStack; MAX_APP_NUM] = [UserStack {
    data: [0; USER_STACK_SIZE],
    sp: USER_STACK_SIZE,
}; MAX_APP_NUM];

impl UserStack {
    pub(crate) fn top_ptr(&self) -> *const u8 {
        self.data.as_ptr().wrapping_add(USER_STACK_SIZE)
    }

    pub(crate) fn sp_ptr(&self) -> *const u8 {
        self.data[self.sp..].as_ptr()
    }
}
