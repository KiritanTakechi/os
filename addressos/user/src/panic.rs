use core::panic::PanicInfo;

use crate::exit;

#[panic_handler]
pub(crate) fn panic(info: &PanicInfo) -> ! {
    let _ = info;
    exit(0);
    unreachable!()
}
