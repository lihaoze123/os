use core::fmt;

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
