use core::{arch::asm, ptr::copy_nonoverlapping, slice::from_raw_parts};

use crate::config::{APP_BASE_ADDRESS, APP_SIZE_LIMIT};

extern "C" {
    static _num_app: usize;
}

fn get_app_num() -> usize {
    unsafe { _num_app }
}

fn get_base_i(app_id: usize) -> usize {
    APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}

fn load_app() {
    extern "C" {
        fn _num_app();
    }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_app_num();

    let app_range = unsafe { from_raw_parts(num_app_ptr.wrapping_add(1), num_app + 1) };

    unsafe {
        asm!("fence.i");
    }

    for i in 0..num_app {
        let base_i = get_base_i(i);
        let app_i = app_range[i];
        let app_i_next = app_range[i + 1];
        let app_i_size = app_i_next - app_i;
        let app_i_ptr = base_i as *mut u8;
        let app_i_src = app_i as *const u8;
        unsafe {
            copy_nonoverlapping(app_i_src, app_i_ptr, app_i_size);
        }
    }
}
