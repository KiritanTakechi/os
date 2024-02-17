#![no_std]
#![no_main]

use syscall::{sys_exit, sys_write};

mod console;
mod fs;
mod panic;
mod syscall;

pub(crate) fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

pub(crate) fn exit(code: usize) -> isize {
    sys_exit(code)
}
