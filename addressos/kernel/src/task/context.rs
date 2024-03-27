use crate::trap::trap_return;

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub fn zero_init() -> Self {
        TaskContext::default()
    }

    pub fn goto_trap_return(kstack_ptr: usize) -> Self {
        TaskContext {
            ra: trap_return as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
