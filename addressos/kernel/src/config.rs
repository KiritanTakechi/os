#![allow(unused)]

use log::Level;

pub const USER_STACK_SIZE: usize = PAGE_SIZE * 4;
pub const KERNEL_STACK_SIZE: usize = PAGE_SIZE * 64;
pub const KERNEL_HEAP_SIZE: usize = PAGE_SIZE * 256;

pub const KERNEL_OFFSET: usize = 0xffffffff80000000;

pub const PHYS_OFFSET: usize = 0xffff800000000000;
pub const ENTRY_COUNT: usize = 512;

pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE_BITS: usize = 0xc;

pub const KVA_START: usize = (usize::MAX) << PAGE_SIZE_BITS;

pub const DEFAULT_LOG_LEVEL: Level = Level::Error;

pub const REAL_TIME_TASK_PRI: u16 = 100;

pub const CLOCK_FREQ: usize = 12500000;
pub const MEMORY_END: usize = 0x8800_0000;

pub const MMIO: &[(usize, usize)] = &[
    (0x0010_0000, 0x00_2000),
];
