#![no_std]
#![no_main]

use power::PowerManager;

mod boot;
mod config;
mod console;
mod panic;
mod power;
mod stack;
mod syscall;

#[no_mangle]
pub(crate) extern "C" fn start_kernel() -> ! {
    PowerManager::shutdown(false)
}