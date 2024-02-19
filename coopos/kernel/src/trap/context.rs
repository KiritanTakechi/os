use riscv::register::sstatus::{self, Sstatus};

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl TrapContext {
    pub(crate) fn new() -> Self {
        Self {
            x: [0; 32],
            sstatus: sstatus::read(),
            sepc: 0,
        }
    }

    pub(crate) fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    pub(crate) fn set_sstatus(&mut self, sstatus: Sstatus) {
        self.sstatus = sstatus;
    }

    pub(crate) fn set_sepc(&mut self, sepc: usize) {
        self.sepc = sepc;
    }
}
