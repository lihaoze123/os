use core::time::Duration;

use crate::{
    time::{Instant, Result},
    yield_,
};

pub fn sleep(dur: Duration) -> Result<()> {
    let start = Instant::now()?;

    while start.elapsed()? < dur {
        yield_();
    }

    Ok(())
}
