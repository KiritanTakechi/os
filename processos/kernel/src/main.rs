#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![feature(ptr_sub_ptr)]
#![feature(const_ptr_sub_ptr)]
#![feature(trivial_bounds)]

use arch::power::shutdown;
use utils::clear_bss;

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate log;

#[macro_use]
pub(crate) mod console;
pub(crate) mod arch;
pub(crate) mod config;
pub(crate) mod ffi;
pub(crate) mod logger;
pub(crate) mod mm;
pub(crate) mod panic;
pub(crate) mod utils;

#[no_mangle]
extern "C" fn start_kernel() -> ! {
    clear_bss();
    logger::init(true).unwrap();
    mm::init();
    shutdown(false);
}
