use crate::config::PAGE_SIZE_BITS;

pub(crate) const PA_WIDTH: usize = 56;
pub(crate) const VA_WIDTH: usize = 39;
pub(crate) const PPN_WIDTH: usize = PA_WIDTH - PAGE_SIZE_BITS;
pub(crate) const VPN_WIDTH: usize = VA_WIDTH - PAGE_SIZE_BITS;
