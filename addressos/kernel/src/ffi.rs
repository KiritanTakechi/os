extern "C" {
    pub(crate) fn sbss();
    pub(crate) fn ebss();
    pub(crate) fn _num_app();
    pub(crate) fn __alltraps();
    pub(crate) fn __restore();
    //pub(crate) fn __switch_to(prev: *mut TaskContext, next: *const TaskContext);
}
