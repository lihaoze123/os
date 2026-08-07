use core::{fmt, time::Duration};

use crate::syscall::sys_clock_gettime;

const NSEC_PER_SEC: i64 = 1_000_000_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum ClockId {
    Realtime = 0,
    Monotonic = 1,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl Timespec {
    pub const ZERO: Self = Self {
        tv_sec: 0,
        tv_nsec: 0,
    };

    pub const fn new(tv_sec: i64, tv_nsec: i64) -> Option<Self> {
        if tv_nsec >= 0 && tv_nsec < NSEC_PER_SEC {
            Some(Self { tv_sec, tv_nsec })
        } else {
            None
        }
    }

    pub const fn is_valid(self) -> bool {
        self.tv_nsec >= 0 && self.tv_nsec < NSEC_PER_SEC
    }

    pub const fn as_secs(self) -> i64 {
        self.tv_sec
    }

    pub const fn subsec_nanos(self) -> u32 {
        self.tv_nsec as u32
    }

    pub fn checked_duration_since(self, earlier: Self) -> Option<Duration> {
        if !self.is_valid() || !earlier.is_valid() {
            return None;
        }

        let self_ns = self.total_nanoseconds();
        let earlier_ns = earlier.total_nanoseconds();
        let difference = self_ns.checked_sub(earlier_ns)?;

        if difference < 0 {
            return None;
        }

        let seconds = difference / i128::from(NSEC_PER_SEC);
        let nanoseconds = difference % i128::from(NSEC_PER_SEC);

        Some(Duration::new(
            u64::try_from(seconds).ok()?,
            nanoseconds as u32,
        ))
    }

    const fn total_nanoseconds(self) -> i128 {
        self.tv_sec as i128 * NSEC_PER_SEC as i128 + self.tv_nsec as i128
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimeError {
    Os(isize),
    InvalidTimespec,
    ClockWentBackwards,
}

impl TimeError {
    pub const fn raw_os_error(self) -> Option<isize> {
        match self {
            Self::Os(errno) => Some(errno),
            Self::InvalidTimespec | Self::ClockWentBackwards => None,
        }
    }
}

impl fmt::Display for TimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Os(errno) => write!(f, "clock_gettime failed with errno {errno}"),
            Self::InvalidTimespec => f.write_str("kernel returned an invalid timespec"),
            Self::ClockWentBackwards => f.write_str("clock moved backwards"),
        }
    }
}

pub type Result<T> = core::result::Result<T, TimeError>;

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
