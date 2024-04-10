use core::slice;

use crate::ffi::{ebss, sbss};

pub(crate) fn clear_bss() {
    unsafe {
        slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}
