use core::panic::PanicInfo;

use crate::{power::PowerManager, println};

#[panic_handler]
pub(crate) fn panic(info: &PanicInfo) -> ! {
    match info.location() {
        Some(location) => {
            println!(
                "panic occurred in file '{}' at line {}",
                location.file(),
                location.line()
            );
        }
        None => {
            println!("panic occurred but can't get location information");
        }
    }
    PowerManager::shutdown(true)
}
