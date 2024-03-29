use alloc::vec::Vec;
use spin::{mutex::SpinMutex, Once};

use crate::loader::{get_app_data, get_num_app};

use self::task::TaskControlBlock;

#[allow(clippy::module_inception)]
pub mod task;
pub mod context;
pub mod switch;

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
        let man =  TaskManager {
            num_app,
            inner: unsafe {
                SpinMutex::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                })
            },
        };

    TASK_MANAGER.call_once(|| {
        man
    });
}

impl TaskManager {
    
}