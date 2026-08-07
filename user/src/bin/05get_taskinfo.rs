#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::get_taskinfo;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("Application B calls get_taskinfo:");
    let task_id = get_taskinfo();
    println!("get_taskinfo returned task id {}", task_id);
    0
}
