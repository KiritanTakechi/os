use sbi_rt::{NoReason, Shutdown, SystemFailure};

pub(crate) struct PowerManager;

impl PowerManager {
    pub fn shutdown(op: i32) -> ! {
        match op {
            0 => sbi_rt::system_reset(Shutdown, NoReason),

            _ => sbi_rt::system_reset(Shutdown, SystemFailure),
        };

        unreachable!()
    }
}
