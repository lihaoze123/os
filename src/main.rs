#![no_main]
#![no_std]

use core::arch::global_asm;

mod lang_items;
mod sbi;
    
#[macro_use]
mod console;

global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello World!");
    panic!("Shutdown machine!");
}

fn clear_bss() {
    unsafe extern "C" {
        safe fn sbss();
        safe fn ebss();
    }
    let lo = (sbss as *const ()).addr();
    let hi = (ebss as *const ()).addr();
    (lo..hi).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}
