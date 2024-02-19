use super::{context::TaskContext, status::TaskStatus};

#[derive(Clone, Copy)]
pub(crate) struct TaskControl {
    context: TaskContext,
    status: TaskStatus,
}

impl TaskControl {
    pub(crate) fn new() -> Self {
        Self {
            context: TaskContext::new(),
            status: TaskStatus::UnInit,
        }
    }

    pub(crate) fn set_context(&mut self, context: TaskContext) {
        self.context = context;
    }

    pub(crate) fn set_status(&mut self, status: TaskStatus) {
        self.status = status
    }
}
