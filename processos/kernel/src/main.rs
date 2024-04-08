#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![feature(ptr_sub_ptr)]
#![feature(const_ptr_sub_ptr)]
#![feature(trivial_bounds)]

use arch::power::shutdown;
use ffi::{ebss, sbss};

#[macro_use]
extern crate alloc;

#[macro_use]
pub(crate) mod console;
pub(crate) mod arch;
pub(crate) mod config;
pub(crate) mod ffi;
pub(crate) mod logger;
pub(crate) mod mm;
pub(crate) mod panic;

#[no_mangle]
extern "C" fn start_kernel() -> ! {
    clear_bss();
    logger::init();
    mm::init();
    shutdown(false);
}

fn clear_bss() {
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}
