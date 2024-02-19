use crate::{
    config::MAX_APP_NUM,
    init::Init,
    loader::get_app_num,
    sync::up::UpSafeCell,
    task::{context::TaskContext, status::TaskStatus},
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
