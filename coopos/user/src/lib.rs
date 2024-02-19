#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

use syscall::{sys_exit, sys_write};

#[macro_use]
pub mod console;
mod fs;
mod panic;
mod syscall;

#[no_mangle]
#[link_section = ".text.entry"]
fn _start() -> ! {
    clear_bss();
    exit(main());
    unreachable!()
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> isize {
    panic!("Cannot find main!");
}

fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf.as_ptr(), buf.len())
}

pub fn exit(error_code: isize) -> isize {
    sys_exit(error_code)
}

pub fn sched_yield() -> isize {
    syscall::sys_sched_yield()
}
