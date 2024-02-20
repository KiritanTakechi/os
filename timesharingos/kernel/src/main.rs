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
mod timer;

#[no_mangle]
pub(crate) extern "C" fn start_kernel() -> ! {
    Init::clear_bss();
    println!("[kernel] Hello, world!");
    trap::init();
    println!("Loading apps... ");
    loader::load_apps();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    println!("Done!");
    task::manager::run_first_task();
    panic!("Unreachable in rust_main!");
}
