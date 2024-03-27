use crate::{
    arch::mm::PageTableFlags,
    config::{kernel_stack_position, PAGE_SIZE, TRAP_CONTEXT},
    mm::{
        address::{PhysPageNum, VirtAddr},
        memory_set::{MapArea, MapType, MemorySet, KERNEL_SPACE},
        option::VirtMemAllocOption,
        page_table::{PageTableEntryTrait, PageTableFlagsTrait},
    },
    trap::{context::TrapContext, trap_handler},
};

use super::context::TaskContext;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    Ready,
    Running,
    Exited,
}

pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub memory_set: MemorySet,
    pub trap_cx_ppn: PhysPageNum,
    pub base_size: usize,
    pub heap_bottom: usize,
    pub program_brk: usize,
}

impl TaskControlBlock {
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }
    // pub fn get_user_token(&self) -> usize {
    //     self.memory_set.token()
    // }
    pub fn new(elf_data: &[u8], app_id: usize) -> Self {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (mut memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
        let trap_cx_ppn = memory_set
            .pt
            .translate(VirtAddr::from(TRAP_CONTEXT))
            .unwrap()
            .phys_page_num();
        let task_status = TaskStatus::Ready;

        let (kernel_stack_bottom, kernel_stack_top) = kernel_stack_position(app_id);

        let stack_area = MapArea::new_with_frames(
            kernel_stack_bottom.into(),
            kernel_stack_top - kernel_stack_bottom,
            PageTableFlags::new()
                .set_readable(true)
                .set_writable(true)
                .set_valid(true),
            MapType::Framed,
            VirtMemAllocOption::new((kernel_stack_top - kernel_stack_bottom) / PAGE_SIZE)
                .alloc()
                .unwrap(),
        );

        KERNEL_SPACE.get_mut().unwrap().lock().map(stack_area);

        let task_control_block = Self {
            task_status,
            task_cx: TaskContext::goto_trap_return(kernel_stack_top),
            memory_set,
            trap_cx_ppn,
            base_size: user_sp,
            heap_bottom: user_sp,
            program_brk: user_sp,
        };
        // // prepare TrapContext in user space
        let trap_cx = task_control_block.get_trap_cx();
        let satp = 8usize << 60
            | KERNEL_SPACE
                .get()
                .unwrap()
                .lock()
                .pt
                .get_root_paddr()
                .floor()
                .0;
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            satp,
            kernel_stack_top,
            trap_handler as usize,
        );
        task_control_block
    }
    // pub fn change_program_brk(&mut self, size: i32) -> Option<usize> {
    //     let old_break = self.program_brk;
    //     let new_brk = self.program_brk as isize + size as isize;
    //     if new_brk < self.heap_bottom as isize {
    //         return None;
    //     }
    //     let result = if size < 0 {
    //         self.memory_set
    //             .shrink_to(VirtAddr(self.heap_bottom), VirtAddr(new_brk as usize))
    //     } else {
    //         self.memory_set
    //             .append_to(VirtAddr(self.heap_bottom), VirtAddr(new_brk as usize))
    //     };
    //     if result {
    //         self.program_brk = new_brk as usize;
    //         Some(old_break)
    //     } else {
    //         None
    //     }
    // }
}
