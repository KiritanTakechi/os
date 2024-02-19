use core::arch::global_asm;

use riscv::register::{
    scause::{self, Exception, Trap},
    stval,
    stvec::{self, TrapMode},
};

use crate::syscall::syscall;

use self::context::TrapContext;

pub(crate) mod context;

global_asm!(include_str!("trap.S"));

pub(crate) fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[no_mangle]
#[repr(align(2))]
pub(crate) fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();

    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
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
