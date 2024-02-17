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
    fn main() -> i32;
}

pub(crate) fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

pub(crate) fn exit(error_code: i32) -> isize {
    sys_exit(error_code)
}
