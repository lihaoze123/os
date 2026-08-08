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

pub fn load_apps() {
    let num_app_ptr = linker_symbol_addr!(_num_app) as *const usize;
    let num_app = unsafe { num_app_ptr.read_volatile() };
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };

    for i in 0..num_app {
        let base_i = get_base_i(i);
        unsafe {
            core::slice::from_raw_parts_mut(base_i as *mut u8, APP_SIZE_LIMIT).fill(0);

            let src = core::slice::from_raw_parts(
                app_start[i] as *const u8,
                app_start[i + 1] - app_start[i],
            );
            let dst =
                core::slice::from_raw_parts_mut(base_i as *mut u8, app_start[i + 1] - app_start[i]);
            dst.copy_from_slice(src);
        }
    }

    unsafe {
        // 确保之后的取指过程中拿到的是修改后的内容
        asm!("fence.i");
    }
}
