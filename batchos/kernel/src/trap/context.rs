use riscv::register::sstatus::{self, set_spp, Sstatus, SPP};

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32], 
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl TrapContext {
    /// set stack pointer to x_2 reg (sp)
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    /// init app context
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let sstatus = sstatus::read(); // CSR sstatus
        unsafe { set_spp(SPP::User) }; //previous privilege mode: user mode
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry, // entry point of app
        };
        cx.set_sp(sp); // app's user stack pointer
        cx // return initial Trap Context of app
    }
}
