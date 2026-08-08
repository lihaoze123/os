use crate::{config::CLOCK_FREQ, time::timer::get_time};

pub mod timer;

const NSEC_PER_SEC: usize = 1_000_000_000;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

pub fn monotonic_now() -> Timespec {
    let ticks = get_time();

    let seconds = ticks / CLOCK_FREQ;
    let remaining_ticks = ticks % CLOCK_FREQ;
    let nanoseconds = (remaining_ticks as u128 * NSEC_PER_SEC as u128 / CLOCK_FREQ as u128) as u64;

    Timespec {
        tv_sec: seconds as i64,
        tv_nsec: nanoseconds as i64,
    }
}
