use crate::{task::manager::{exit_current_and_run_next, suspend_current_and_run_next}, timer::get_time_us};

pub(crate) fn sys_exit(error_code: isize) -> ! {
    println!("[kernel] Application exited with code {}", error_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_sched_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time() -> isize {
    get_time_us() as isize
}