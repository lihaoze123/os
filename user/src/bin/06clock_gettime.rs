// 运行一个递归斐波那契，测量运行时间

#![no_std]
#![no_main]

use user_lib::time::Instant;

#[macro_use]
extern crate user_lib;

const MOD: u64 = 10007;

fn fib(x: u64) -> u64 {
    if x <= 1 {
        1
    } else {
        (fib(x - 1) + fib(x - 2)) % MOD
    }
}

#[unsafe(no_mangle)]
fn main() -> i32 {
    let start = Instant::now().unwrap();
    println!("fib(35) % {} = {}", MOD, fib(35));
    let elapsed = start.elapsed().unwrap().as_micros();
    println!(
        "elapsed: {}.{} ms",
        elapsed / 1_000_000,
        elapsed % 1_000_000
    );
    0
}
