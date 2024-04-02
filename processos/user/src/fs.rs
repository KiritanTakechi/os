use num_enum::IntoPrimitive;

#[derive(IntoPrimitive)]
#[repr(usize)]
pub(crate) enum FileSystem {
    Stdout = 1,
}