use core::time::Duration;

const NSEC_PER_SEC: i64 = 1_000_000_000;

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
