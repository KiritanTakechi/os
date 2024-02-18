use core::arch::asm;

use riscv::register::sstatus::{self, set_spp, Sstatus, SPP};

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl TrapContext {
    pub(crate) fn new(entry: usize) -> Self {
        Self {
            x: [0; 32],
            sstatus: sstatus::read(),
            sepc: entry,
        }
    }

    pub(crate) fn set_usermode(&mut self) {
        unsafe { set_spp(SPP::User) };
    }

    pub(crate) fn set_kernelmode(&mut self) {
        unsafe { set_spp(SPP::Supervisor) };
    }

    pub(crate) fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    pub(crate) fn save_register(&mut self) {
        unsafe {
            let self_ptr = self as *mut TrapContext;
            asm!(
                "sd x0, 0*8({0})",
                "sd x1, 1*8({0})",
                "sd x2, 2*8({0})",
                "sd x3, 3*8({0})",
                "sd x4, 4*8({0})",
                "sd x5, 5*8({0})",
                "sd x6, 6*8({0})",
                "sd x7, 7*8({0})",
                "sd x8, 8*8({0})",
                "sd x9, 9*8({0})",
                "sd x10, 10*8({0})",
                "sd x11, 11*8({0})",
                "sd x12, 12*8({0})",
                "sd x13, 13*8({0})",
                "sd x14, 14*8({0})",
                "sd x15, 15*8({0})",
                "sd x16, 16*8({0})",
                "sd x17, 17*8({0})",
                "sd x18, 18*8({0})",
                "sd x19, 19*8({0})",
                "sd x20, 20*8({0})",
                "sd x21, 21*8({0})",
                "sd x22, 22*8({0})",
                "sd x23, 23*8({0})",
                "sd x24, 24*8({0})",
                "sd x25, 25*8({0})",
                "sd x26, 26*8({0})",
                "sd x27, 27*8({0})",
                "sd x28, 28*8({0})",
                "sd x29, 29*8({0})",
                "sd x30, 30*8({0})",
                "sd x31, 31*8({0})",
                in(reg) self_ptr,
            )
        }
        self.sstatus = sstatus::read();
        self.sepc = riscv::register::sepc::read();
    }
}
