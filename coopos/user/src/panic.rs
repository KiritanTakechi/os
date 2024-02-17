use core::panic::PanicInfo;

use crate::exit;

#[panic_handler]
pub(crate) fn panic(info: &PanicInfo) -> ! {
    exit(0);
    unreachable!()
}
