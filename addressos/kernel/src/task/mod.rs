use alloc::vec::Vec;
use spin::{mutex::SpinMutex, Once};

use crate::{
    arch::power::shutdown, loader::{get_app_data, get_num_app}, task::{context::TaskContext, switch::__switch, task::TaskStatus}, trap::context::TrapContext
};

use self::task::TaskControlBlock;

pub mod context;
pub mod switch;
#[allow(clippy::module_inception)]
pub mod task;

pub struct TaskManager {
    num_app: usize,
    inner: SpinMutex<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: Vec<TaskControlBlock>,
    current_task: usize,
}

pub static TASK_MANAGER: Once<TaskManager> = Once::new();

pub fn init(num_app: usize) {
    println!("init TASK_MANAGER");
    let num_app = get_num_app();
    println!("num_app = {}", num_app);

    let mut tasks: Vec<TaskControlBlock> = Vec::new();
    for i in 0..num_app {
        //todo!()
        tasks.push(TaskControlBlock::new(get_app_data(i), i));
    }
    let man = TaskManager {
        num_app,
        inner: unsafe {
            SpinMutex::new(TaskManagerInner {
                tasks,
                current_task: 0,
            })
        },
    };

    TASK_MANAGER.call_once(|| man);
}

impl TaskManager {
    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.lock();
        let next_task = &mut inner.tasks[0];
        next_task.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &next_task.task_cx as *const TaskContext;
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        unsafe {
            __switch(&mut _unused as *mut _, next_task_cx_ptr);
        }
        panic!("unreachable in run_first_task!");
    }

    fn mark_current_suspended(&self) {
        let mut inner = self.inner.lock();
        let cur = inner.current_task;
        inner.tasks[cur].task_status = TaskStatus::Ready;
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.lock();
        let cur = inner.current_task;
        inner.tasks[cur].task_status = TaskStatus::Exited;
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.lock();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    fn get_current_token(&self) -> usize {
        let inner = self.inner.lock();
        inner.tasks[inner.current_task].get_user_token()
    }

    /// Get the current 'Running' task's trap contexts.
    fn get_current_trap_cx(&self) -> &'static mut TrapContext {
        let inner = self.inner.lock();
        inner.tasks[inner.current_task].get_trap_cx()
    }

    // Change the current 'Running' task's program break
    // pub fn change_current_program_brk(&self, size: i32) -> Option<usize> {
    //     let mut inner = self.inner.lock();
    //     let cur = inner.current_task;
    //     inner.tasks[cur].change_program_brk(size)
    // }

    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.lock();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            drop(inner);
            // before this, we should drop local variables that must be dropped manually
            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
            // go back to user mode
        } else {
            println!("All applications completed!");
            shutdown(false);
        }
    }
}

pub fn run_first_task() {
    TASK_MANAGER.get().unwrap().run_first_task();
}

/// Switch current `Running` task to the task we have found,
/// or there is no `Ready` task and we can exit with all applications completed
fn run_next_task() {
    TASK_MANAGER.get().unwrap().run_next_task();
}

/// Change the status of current `Running` task into `Ready`.
fn mark_current_suspended() {
    TASK_MANAGER.get().unwrap().mark_current_suspended();
}

/// Change the status of current `Running` task into `Exited`.
fn mark_current_exited() {
    TASK_MANAGER.get().unwrap().mark_current_exited();
}

/// Suspend the current 'Running' task and run the next task in task list.
pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

/// Exit the current 'Running' task and run the next task in task list.
pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

/// Get the current 'Running' task's token.
pub fn current_user_token() -> usize {
    TASK_MANAGER.get().unwrap().get_current_token()
}

/// Get the current 'Running' task's trap contexts.
pub fn current_trap_cx() -> &'static mut TrapContext {
    TASK_MANAGER.get().unwrap().get_current_trap_cx()
}

// Change the current 'Running' task's program break
// pub fn change_program_brk(size: i32) -> Option<usize> {
//     TASK_MANAGER.get().unwrap().change_current_program_brk(size)
// }
