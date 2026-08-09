mod clock;
mod error;
mod instant;
mod sleep;
mod system_time;
mod timespec;

pub use core::time::Duration;

pub use clock::{ClockId, clock_gettime, monotonic_now, realtime_now};
pub use error::{Result, TimeError};
pub use instant::Instant;
pub use sleep::sleep;
pub use system_time::{SystemTime, UNIX_EPOCH};
pub use timespec::Timespec;
