#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

fn sqrt(x: f64) -> f64 {
    let mut ans = x;
    for _ in 0..1000 {
        ans -= (ans * ans - x) / 2.0 / ans;
    }
    ans
}

#[unsafe(no_mangle)]
fn main() -> i32 {
    let mut sum = 0.0;
    for n in 1..=10_000_000 {
        sum += 1.0 / (n as f64 * n as f64);
    }
    let pi = sqrt(6.0 * sum);
    println!("pi = {}", pi);

    0
}
