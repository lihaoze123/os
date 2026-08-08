use core::time::Duration;

use super::{
    clock::monotonic_now,
    error::{Result, TimeError},
    timespec::Timespec,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Instant(Timespec);

impl Instant {
    pub fn now() -> Result<Self> {
        monotonic_now().map(Self)
    }

    pub const fn as_timespec(self) -> Timespec {
        self.0
    }

    pub fn checked_duration_since(self, earlier: Self) -> Option<Duration> {
        self.0.checked_duration_since(earlier.0)
    }

    pub fn duration_since(self, earlier: Self) -> Result<Duration> {
        self.checked_duration_since(earlier)
            .ok_or(TimeError::ClockWentBackwards)
    }

    pub fn elapsed(self) -> Result<Duration> {
        Self::now()?.duration_since(self)
    }
}
