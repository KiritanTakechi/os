use riscv::register::stvec::{self, TrapMode};

pub(crate) mod context;

pub(crate) fn init() {
    
    unsafe {
        stvec::write(trap_handler as usize , TrapMode::Direct);
    }
}

#[no_mangle]
pub(crate) fn trap_handler() {
    todo!()
}