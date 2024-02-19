use core::{
    slice::from_raw_parts,
    str::from_utf8_unchecked,
};

const FD_STDOUT: usize = 1;

pub(crate) fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let slice = unsafe { from_raw_parts(buf, len) };
            let str = unsafe { from_utf8_unchecked(slice) };
            print!("{}", str);
            len as isize
        }
        _ => {
            panic!("sys_write: unsupported file descriptor: {}", fd);
        }
    }
}
