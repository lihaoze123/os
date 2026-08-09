use crate::syscall::sys_clock_gettime;

use super::{
    error::{Result, TimeError},
    timespec::Timespec,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum ClockId {
    Realtime = 0,
    Monotonic = 1,
}

pub fn clock_gettime(clock: ClockId) -> Result<Timespec> {
    let mut time = Timespec::ZERO;
    let result = sys_clock_gettime(clock as usize, &mut time);

    if result < 0 {
        return Err(TimeError::Os(result.saturating_neg()));
    }
    if result != 0 || !time.is_valid() {
        return Err(TimeError::InvalidTimespec);
    }

    Ok(time)
}

pub fn realtime_now() -> Result<Timespec> {
    clock_gettime(ClockId::Realtime)
}

pub fn monotonic_now() -> Result<Timespec> {
    clock_gettime(ClockId::Monotonic)
}
