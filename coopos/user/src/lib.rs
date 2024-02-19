#![no_std]
#![no_main]

use syscall::{sys_exit, sys_write};

#[macro_use]
pub mod console;
mod fs;
mod panic;
mod syscall;

#[no_mangle]
fn _start() -> ! {
    exit(unsafe { main() });
    unreachable!()
}

extern "C" {
    fn main() -> isize;
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