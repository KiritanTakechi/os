use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};

pub fn shutdown(failure: bool) -> ! {
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}