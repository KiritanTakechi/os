use riscv::register::time;

use crate::config::CLOCK_FREQ;

const SBI_SET_TIMER: usize = 0;
const TICKS_PER_SEC: usize = 100;

pub fn set_timer(timer: usize) {
    sbi_rt::set_timer(timer as _);
}

pub(crate) fn get_time() -> usize {
    time::read()
}

pub(crate) fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

const MICRO_PER_SEC: usize = 1_000_000;

pub fn get_time_us() -> usize {
    time::read() / (CLOCK_FREQ / MICRO_PER_SEC)
}