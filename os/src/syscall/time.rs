use crate::time::{Timespec, monotonic_now};

const CLOCK_REALTIME: usize = 0;
const CLOCK_MONOTONIC: usize = 1;

const EFAULT: isize = 14;
const EINVAL: isize = 22;

pub fn sys_clock_gettime(clk_id: usize, tp_addr: usize) -> isize {
    if tp_addr == 0 {
        return -EFAULT;
    }

    let time = match clk_id {
        CLOCK_MONOTONIC => monotonic_now(),
        CLOCK_REALTIME => return -EINVAL,
        _ => return -EINVAL,
    };

    let tp = tp_addr as *mut Timespec;
    unsafe {
        tp.write_unaligned(time);
    }

    0
}
