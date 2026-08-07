use crate::sync::up::UPSafeCell;

include!(env!("OS_APP_NAMES_RS"));

pub(super) const MAX_APP_NUM: usize = 16;

pub(super) struct AppManager {
    num_app: usize,
    current_app: usize,
}

impl AppManager {
    pub(super) fn get_current_app(&self) -> Option<usize> {
        if self.current_app >= self.num_app {
            None
        } else {
            Some(self.current_app)
        }
    }

    pub(super) fn get_current_task_info(&self) -> (usize, &'static str) {
        let task_id = self
            .current_app
            .checked_sub(1)
            .expect("no application is currently running");
        (task_id, APP_NAMES[task_id])
    }

    pub(super) fn move_to_next_app(&mut self) {
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
            AppManager {
                num_app,
                current_app: 0,
            }
        })
    };
}
