#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

use arch::power::shutdown;

mod arch;
mod config;
#[macro_use]
mod console;
mod mm;
mod panic;
mod sync;

#[no_mangle]
extern "C" fn start_kernel() -> ! {
    shutdown(false)
}

fn init() {
    mm::init();
}
