use core::time::Duration;

use super::{
    clock::realtime_now,
    error::{Result, TimeError},
    timespec::Timespec,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SystemTime(Timespec);

pub const UNIX_EPOCH: SystemTime = SystemTime(Timespec::ZERO);

impl SystemTime {
    pub fn now() -> Result<Self> {
        realtime_now().map(Self)
    }

    pub const fn as_timespec(self) -> Timespec {
        self.0
    }

    pub fn duration_since(self, earlier: Self) -> Result<Duration> {
        self.0
            .checked_duration_since(earlier.0)
            .ok_or(TimeError::ClockWentBackwards)
    }
}
