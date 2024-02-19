#![no_std]
#![no_main]
#![feature(fn_align)]

use init::Init;
use power::PowerManager;

mod boot;
mod ffi;
mod config;
#[macro_use]
mod console;
mod loader;
mod panic;
mod power;
mod stack;
mod syscall;
mod trap;
mod task;
mod init;
mod sync;

#[no_mangle]
pub(crate) extern "C" fn start_kernel() -> ! {
    Init::clear_bss();
    PowerManager::shutdown(false)
}
