#![no_std]
#![no_main]

use user_lib::time::{self, Duration, Instant};

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
fn main() -> i32 {
    let duration = Duration::from_secs(3);
    let start = Instant::now().unwrap();
    println!("[sleep] Test sleep start!");

    time::sleep(duration).unwrap();

    let elapsed = start.elapsed().unwrap();
    println!("[sleep] Test sleep elapsed: {:?}", elapsed);
    println!("[sleep] Test sleep OK!");
    0
}
