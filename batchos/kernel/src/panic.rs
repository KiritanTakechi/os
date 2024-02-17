use core::panic::PanicInfo;

use crate::power::PowerManager;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    PowerManager::shutdown(-1)
}