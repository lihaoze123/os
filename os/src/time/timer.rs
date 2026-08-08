use riscv::register::time;

use crate::{config::CLOCK_FREQ, sbi::set_timer};

const TIMER_HZ: usize = 100;

pub fn get_time() -> usize {
    time::read()
}

pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TIMER_HZ);
}
