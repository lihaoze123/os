use core::arch::asm;

pub unsafe fn print_stack_trace() -> () {
    let mut fp: *const usize;
    unsafe {
        asm!("mv {}, fp", out(reg) fp);
    }

    log::debug!("== Begin stack trace ==");
    while !fp.is_null() {
        unsafe {
            let saved_ra = *fp.sub(1);
            let saved_fp = *fp.sub(1);

            log::debug!("0x{:016x}, fp = 0x{:016x}", saved_ra, saved_fp);
            fp = saved_fp as *const usize;
        }
    }
    log::debug!("== End stack trace ==");
}
