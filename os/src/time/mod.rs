const CLOCK_REALTIME: usize = 0;
const CLOCK_MONOTONIC: usize = 1;

const CLOCK_FREQ: u64 = 10_000_000;
const NSEC_PER_SEC: u64 = 1_000_000_000;

const EFAULT: isize = 14;
const EINVAL: isize = 22;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

fn monotonic_now() -> Timespec {
    let ticks = riscv::register::time::read64();

    let seconds = ticks / CLOCK_FREQ;
    let remaining_ticks = ticks % CLOCK_FREQ;
    let nanoseconds = (remaining_ticks as u128 * NSEC_PER_SEC as u128 / CLOCK_FREQ as u128) as u64;

    Timespec {
        tv_sec: seconds as i64,
        tv_nsec: nanoseconds as i64,
    }
}

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
