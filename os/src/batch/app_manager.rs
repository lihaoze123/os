use core::arch::asm;

use crate::{sbi::shutdown, sync::up::UPSafeCell};

include!(concat!(env!("OUT_DIR"), "/app_names.rs"));

pub(super) const MAX_APP_NUM: usize = 16;
pub(super) const APP_BASE_ADDRESS: usize = 0x80400000;
pub(super) const APP_SIZE_LIMIT: usize = 0x20000;

pub(super) struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    pub fn print_app_info(&self) {
        log::info!("[kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            log::info!(
                "[kernel] app_{} [{:#x}, {:#x}]",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }

    pub(super) fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            log::info!("All applications completed!");
            shutdown(false);
        }
        log::info!("[kernel] Loading app_{}", app_id);
        unsafe {
            // clear the app area
            core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);

            let app_src = core::slice::from_raw_parts(
                self.app_start[app_id] as *const u8,
                self.app_start[app_id + 1] - self.app_start[app_id],
            );
            let app_dst = core::slice::from_raw_parts_mut(
                APP_BASE_ADDRESS as *mut u8,
                self.app_start[app_id + 1] - self.app_start[app_id],
            );
            app_dst.copy_from_slice(app_src);

            // 确保之后的取指过程中拿到的是修改后的内容
            asm!("fence.i");
        }
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn get_current_task_info(&self) -> (usize, &'static str) {
        let task_id = self
            .current_app
            .checked_sub(1)
            .expect("no application is currently running");
        (task_id, APP_NAMES[task_id])
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }
}

lazy_static::lazy_static! {
    pub(super) static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            unsafe extern "C" {
                safe fn _num_app();
            }
            let num_app_ptr = linker_symbol_addr!(_num_app) as * const usize;
            let num_app = num_app_ptr.read_volatile();
            let mut app_start = [0usize; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] = core::slice::from_raw_parts(
                num_app_ptr.add(1), num_app + 1
            );
            app_start[..=num_app].copy_from_slice(app_start_raw);
            AppManager {
                num_app,
                current_app: 0,
                app_start,
            }
        })
    };
}
