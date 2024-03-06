#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![feature(ptr_sub_ptr)]
#![feature(const_ptr_sub_ptr)]
#![feature(trivial_bounds)]

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
    clear_bss();
    println!("[kernel] Hello, world!");
    mm::init();
    println!("[kernel] back to world!");
    shutdown(false)
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
