use crate::sbi::shutdown;

use self::{app_manager::APP_MANAGER, stack::init_app_cx};

mod app_manager;
mod stack;

pub fn get_current_task_info() -> (usize, &'static str) {
    APP_MANAGER.exclusive_access().get_current_task_info()
}

pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = match app_manager.get_current_app() {
        Some(id) => id,
        None => {
            log::info!("All applications completed");
            shutdown(false);
        }
    };
    app_manager.move_to_next_app();
    drop(app_manager);

    unsafe extern "C" {
        unsafe fn __restore(cx_addr: usize);
    }

    unsafe {
        __restore(init_app_cx(current_app));
    }
    unreachable!()
}
