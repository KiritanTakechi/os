#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod riscv;
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub(crate) use self::riscv::*;

#[cfg(target_arch = "x86_64")]
mod x86;
#[cfg(target_arch = "x86_64")]
pub(crate) use self::x86::*;
