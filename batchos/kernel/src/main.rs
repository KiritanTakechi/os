#![no_std]
#![no_main]
#![feature(panic_info_message)]

global_asm!(include_str!("link_app.S"));

use core::arch::global_asm;

use batch::APP_MANAGER;
use power::PowerManager;

mod batch;
mod boot;
pub mod panic;
mod power;
mod sync;
mod tty;
mod trap;
mod syscall;

#[no_mangle]
extern "C" fn start_kernel() -> ! {
    clear_bss();
    trap::init();
    batch::init();
    batch::run_next_app();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}