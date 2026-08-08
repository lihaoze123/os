use crate::{
    syscall::{
        fs::sys_write,
        process::{sys_exit, sys_yield},
    },
    time::sys_clock_gettime,
};

mod fs;
mod process;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_CLOCK_GETTIME: usize = 113;
const SYSCALL_YIELD: usize = 124;

pub fn syscall(id: usize, args: [usize; 3]) -> isize {
    match id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_CLOCK_GETTIME => sys_clock_gettime(args[0], args[1]),
        SYSCALL_YIELD => sys_yield(),
        _ => panic!("Unsupported syscall_id: {}", id),
    }
}
