use crate::config::PAGE_SIZE;

pub(crate) mod address;
mod frame;
mod frame_allocator;
mod heap_allocator;
pub mod memory_set;
mod option;
pub(crate) mod page_table;

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    //heap_test();
    //frame_allocator_test();
    //option::frame_allocator_test();
    memory_set::init();
    //memory_set::remap_test();
}

pub const fn is_page_aligned(p: usize) -> bool {
    (p & (PAGE_SIZE - 1)) == 0
}
