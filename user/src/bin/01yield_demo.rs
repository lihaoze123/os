#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::yield_;

const YIELD_ROUNDS: usize = 3;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("[yield-demo] started");

    for round in 1..=YIELD_ROUNDS {
        println!("[yield-demo] round {}: yielding the CPU", round);
        let result = yield_();
        println!(
            "[yield-demo] round {}: resumed, yield returned {}",
            round, result
        );
    }

    println!("[yield-demo] finished");
    0
}
