use crate::config::{KERNEL_STACK_SIZE, USER_STACK_SIZE};

#[derive(Copy, Clone)]
#[repr(align(4096))]
pub(crate) struct KernelStack {
    stack: [u8; KERNEL_STACK_SIZE],
}

pub(crate) static KERNEL_STACK: KernelStack = KernelStack {
    stack: [0; KERNEL_STACK_SIZE],
};

impl KernelStack {
    pub(crate) fn top(&self) -> *const u8 {
        self.stack.as_ptr().wrapping_add(KERNEL_STACK_SIZE)
    }
}

#[derive(Copy, Clone)]
#[repr(align(4096))]
pub(crate) struct UserStack {
    stack: [u8; USER_STACK_SIZE],
}

pub(crate) static USER_STACK: UserStack = UserStack {
    stack: [0; USER_STACK_SIZE],
};

impl UserStack {
    pub(crate) fn top(&self) -> *const u8 {
        self.stack.as_ptr().wrapping_add(USER_STACK_SIZE)
    }
}