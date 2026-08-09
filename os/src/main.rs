#![no_main]
#![no_std]

macro_rules! linker_symbol_addr {
    ($symbol:path) => {
        ($symbol as *const ()).addr()
    };
}

use core::arch::global_asm;

mod config;
mod lang_items;
mod loader;
mod logging;
mod sbi;
mod stack_trace;
mod sync;
mod syscall;
mod task;
mod time;
mod trap;

#[macro_use]
mod console;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!(env!("OS_LINK_APP_ASM")));

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    log::info!("Hello from system!");

    trap::init();
    loader::load_apps();

    trap::enable_timer_interrupt();
    time::timer::set_next_trigger();
    task::run_first_task();

    unreachable!()
}

fn clear_bss() {
    unsafe extern "C" {
        safe fn sbss();
        safe fn ebss();
    }
    let lo = linker_symbol_addr!(sbss);
    let hi = linker_symbol_addr!(ebss);
    (lo..hi).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}
