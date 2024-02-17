#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

use core::slice::from_raw_parts_mut;

use syscall::{sys_exit, sys_write};

#[macro_use]
pub mod console;
mod syscall;
mod panic;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    //panic!("unreachable after sys_exit!");
    unreachable!()
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    println!("Hello, world!");
    //panic!("Cannot find main!");
    0
}

pub(crate) fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

pub(crate) fn exit(code: i32) -> isize {
    sys_exit(code)
}

fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }

    unsafe {
        from_raw_parts_mut(
            start_bss as usize as *mut u8,
            end_bss as usize - start_bss as usize,
        )
        .fill(0);
    }

    // (start_bss as usize..end_bss as usize).for_each(|addr| unsafe {
    //     (addr as *mut u8).write_volatile(0);
    // });
}