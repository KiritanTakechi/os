use crate::{
    config::MAX_APP_NUM, ffi::{__restore, __switch_to}, init::Init, loader::get_app_num, power::PowerManager, sync::up::UpSafeCell, task::{context::TaskContext, status::TaskStatus}
};

use super::control::TaskControl;
use lazy_static::*;

pub(crate) struct TaskManager {
    app_num: usize,
    inner: UpSafeCell<TaskManagerInner>,
}

pub(crate) struct TaskManagerInner {
    tasks: [TaskControl; MAX_APP_NUM],
    current_task: usize,
}

lazy_static! {
    static ref TASK_MANAGER: TaskManager = {
        let app_num = get_app_num();
        let task_control = TaskControl::new();

        let mut tasks = [task_control; MAX_APP_NUM];

        tasks.iter_mut().enumerate().for_each(|(index, task)| {
            task.set_context(TaskContext::goto_restore(Init::init_app_cx(index)));
            task.set_status(TaskStatus::Ready);
        });

        TaskManager {
            app_num,
            inner: UpSafeCell::new(TaskManagerInner {
                tasks,
                current_task: 0,
            }),
        }
    };
}

impl TaskManager {
    fn run_first_task(&self) {
        let mut inner = self.inner.borrow_mut();
        let first_task = &mut inner.tasks[0];
        first_task.set_status(TaskStatus::Running);

        let first_task_context_ptr = &first_task.get_context() as *const TaskContext;
        drop(inner);

        let unset_context = &mut TaskContext::new() as *mut TaskContext;

        unsafe { __switch_to(unset_context, first_task_context_ptr) }

        panic!("unreachable in run_first_task!");
    }

    fn mark_current_suspended(&self) {
        let mut inner = TASK_MANAGER.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].set_status(TaskStatus::Ready);
    }

    fn mark_current_exited(&self) {
        let mut inner = TASK_MANAGER.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].set_status(TaskStatus::Exited);
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = TASK_MANAGER.inner.borrow_mut();
        let current = inner.current_task;
        (current + 1..self.app_num)
            .chain(1..current)
            .find(|&index| {
                let task = &inner.tasks[index];
                task.get_status() == TaskStatus::Ready
            })
    }

    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.borrow_mut();
            let current = inner.current_task;
            inner.tasks[next].set_status(TaskStatus::Running);
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].get_context() as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].get_context() as *const TaskContext;
            drop(inner);

            unsafe {
                __switch_to(current_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            println!("All applications completed!");
            PowerManager::shutdown(false);
        }
    }
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}
