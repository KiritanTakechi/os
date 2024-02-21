#![no_std]
#![no_main]
#![feature(panic_info_message)]

use arch::power::shutdown;

mod arch;
mod panic;

#[no_mangle]
extern "C" fn start_kernel() -> ! {
    shutdown(false)
}