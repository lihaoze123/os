use alloc::string::String;

use crate::{mem::user_buffer::UserBuffer, print, task::current_user_token};

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let buffer = UserBuffer::new(current_user_token(), buf, len).bytes();
            let str = String::from_utf8_lossy_owned(buffer);
            print!("{}", str);
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}
