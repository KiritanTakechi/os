use core::{arch::global_asm, slice::from_raw_parts_mut};

use riscv::register::sstatus::{self, set_spp, SPP};

use crate::{ffi::{ebss, sbss}, loader::get_base_i, stack::{KERNEL_STACK, USER_STACK}, trap::context::TrapContext};

global_asm!(include_str!("link_app.S"));

pub(crate) struct Init;

impl Init {
    pub(crate) fn clear_bss() {
        unsafe {
            from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
                .fill(0);
        }
    }

    pub(crate) fn init_app_cx(app_id: usize) -> usize {
        unsafe { set_spp(SPP::User) };
        let mut cx = TrapContext::new();
        let sp_ptr = USER_STACK[app_id].top_ptr();
        let sepc_ptr = get_base_i(app_id);
        cx.set_sp(sp_ptr);
        cx.set_sepc(sepc_ptr);

        KERNEL_STACK[app_id].push_context(cx)
    }
}