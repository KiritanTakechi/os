use self::{frame_allocator::frame_allocator_test, heap_allocator::heap_test};

pub(crate) mod address;
mod frame;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod option;
pub(crate) mod page_table;

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    heap_test();
    frame_allocator_test();
}
