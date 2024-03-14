#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
    InvalidArgs,
    NoMemory,
    PageFault,
    AccessDenied,
    IoError,
    NotEnoughResources,
}
