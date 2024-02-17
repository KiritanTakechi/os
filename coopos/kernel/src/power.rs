use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};

pub(crate) struct PowerManager;

impl PowerManager {
    pub(crate) fn shutdown(is_err: bool) -> ! {
        match !is_err {
            true => {
                system_reset(Shutdown, NoReason);
            }
            false => {
                system_reset(Shutdown, SystemFailure);
            }
        }
        unreachable!()
    }
}
