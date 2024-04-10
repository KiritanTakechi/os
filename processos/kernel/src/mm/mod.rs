pub(crate) mod address;
mod frame;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod option;
mod page_table;
mod space;

pub(super) fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();

    tests::test();
}

mod tests;
