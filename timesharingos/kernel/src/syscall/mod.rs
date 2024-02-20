use core::usize;

use num_enum::{IntoPrimitive, TryFromPrimitive};

use self::{fs::sys_write, process::{sys_exit, sys_get_time, sys_sched_yield}};

mod fs;
mod process;

#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(usize)]
pub(crate) enum Syscall {
    Write = 64,
    Exit = 93,
    SchedYield = 124,
    GetTime = 169,
}

pub(crate) fn syscall (syscall_id: usize, args: [usize; 3]) -> isize {
    match Syscall::try_from(syscall_id) {
        Ok(Syscall::Write) => {
            sys_write(args[0], args[1] as *const u8, args[2])
        }
        Ok(Syscall::Exit) => {
            sys_exit(args[0] as isize)
        }
        Ok(Syscall::SchedYield) => {
            sys_sched_yield()
        }
        Ok(Syscall::GetTime) => {
            sys_get_time()
        }
        Err(e) => {
            panic!("syscall_id not found: {:?}", e);
        }
    }
}