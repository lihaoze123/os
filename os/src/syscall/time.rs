use crate::{mem::user_buffer::UserBuffer, task::current_user_token, time::monotonic_now};

const CLOCK_REALTIME: usize = 0;
const CLOCK_MONOTONIC: usize = 1;

const EFAULT: isize = 14;
const EINVAL: isize = 22;

pub fn sys_clock_gettime(clk_id: usize, tp_addr: *const u8) -> isize {
    if tp_addr.is_null() {
        return -EFAULT;
    }

    let time = match clk_id {
        CLOCK_MONOTONIC => monotonic_now(),
        CLOCK_REALTIME => return -EINVAL,
        _ => return -EINVAL,
    };

    let mut buffer = UserBuffer::new(current_user_token(), tp_addr, size_of_val(&time));
    unsafe { buffer.write_raw_value_unchecked(time) };

    0
}
