use core::arch::asm;

use num_enum::IntoPrimitive;

pub(crate) fn syscall(id: usize, args: [usize; 3]) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("a0") args[0] => ret,
            in("a1") args[1],
            in("a2") args[2],
            in("a7") id,
        );
    }
    ret
}

#[derive(IntoPrimitive)]
#[repr(usize)]
enum Syscall {
    Write = 64,
    Exit = 93,
}


pub(crate) fn sys_write(fd: usize, buf: &[u8]) -> isize {
    syscall(Syscall::Write.into(), [fd, buf.as_ptr() as usize, buf.len()])
}

pub(crate) fn sys_exit(error_code: i32) -> isize {
    syscall(Syscall::Exit.into(), [error_code as usize, 0, 0])
}