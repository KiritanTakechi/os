pub(crate) const KERNEL_STACK_SIZE: usize = 4096 << 1;
pub(crate) const USER_STACK_SIZE: usize = 4096 << 1;
pub(crate) const MAX_APP_NUM: usize = 4;
pub(crate) const APP_BASE_ADDRESS: usize = 0x80400000;
pub(crate) const APP_SIZE_LIMIT: usize = 0x20000;
