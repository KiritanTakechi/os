use crate::ffi::__restore;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub(crate) fn new() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    pub(crate) fn goto_restore(kerner_stack_ptr: usize) -> Self {
        Self {
            ra: __restore as usize,
            sp: kerner_stack_ptr,
            s: [0; 12],
        }
    }
}
