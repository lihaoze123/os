use core::arch::asm;

pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x20000;

#[inline]
pub fn get_base_i(app_id: usize) -> usize {
    APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}

unsafe extern "C" {
    safe fn _num_app();
}

#[inline]
pub fn get_num_app() -> usize {
    let num_app_ptr = linker_symbol_addr!(_num_app) as *const usize;
    unsafe { num_app_ptr.read_volatile() }
}

pub fn get_app_data(app_id: usize) -> &'static [u8] {
    let num_app_ptr = linker_symbol_addr!(_num_app) as *const usize;
    let num_app = unsafe { num_app_ptr.read_volatile() };
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };
    assert!(app_id < num_app);
    unsafe {
        core::slice::from_raw_parts(
            app_start[app_id] as *const u8,
            app_start[app_id + 1] - app_start[app_id],
        )
    }
}
