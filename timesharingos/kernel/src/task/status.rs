#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
