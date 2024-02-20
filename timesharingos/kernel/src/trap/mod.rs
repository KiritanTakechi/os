use core::arch::global_asm;

use riscv::register::{
    scause::{self, Exception, Interrupt, Trap}, sie, stval, stvec::{self, TrapMode}
};

use crate::{ffi::__alltraps, syscall::syscall, task::manager::suspend_current_and_run_next, timer::set_next_trigger};

use self::context::TrapContext;

pub(crate) mod context;

global_asm!(include_str!("trap.S"));

pub(crate) fn init() {
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

#[no_mangle]
pub(crate) fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();

    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
        Trap::Exception(Exception::StoreFault) => {
            panic!("store fault at {:#x}", stval);
        },
        Trap::Exception(Exception::IllegalInstruction) => {
            panic!("illegal instruction at {:#x}", cx.sepc);
        }
        _ => {
            panic!(
                "unhandled exception! \n{:?}\n stval: {:#x}",
                scause.cause(),
                stval
            );
        }
    }
    cx
}
