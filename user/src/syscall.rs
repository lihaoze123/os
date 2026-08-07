use core::arch::asm;

use crate::time::Timespec;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_CLOCK_GETTIME: usize = 113;
const SYSCALL_GET_TASKINFO: usize = 410;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

pub fn sys_get_taskinfo() -> isize {
    syscall(SYSCALL_GET_TASKINFO, [0, 0, 0])
}

pub(crate) fn sys_clock_gettime(clock_id: usize, ts: &mut Timespec) -> isize {
    syscall(
        SYSCALL_CLOCK_GETTIME,
        [clock_id, ts as *mut Timespec as usize, 0],
    )
}
