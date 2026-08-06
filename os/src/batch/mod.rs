mod app_manager;
mod stack;

use app_manager::APP_MANAGER;

use crate::{
    batch::{
        app_manager::APP_BASE_ADDRESS,
        stack::{KERNEL_STACK, USER_STACK},
    },
    trap::TrapContext,
};

pub fn init() {
    print_app_info();
}

pub fn print_app_info() {
    APP_MANAGER.exclusive_access().print_app_info();
}

pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();

    app_manager.load_app(current_app);
    app_manager.move_to_next_app();
    drop(app_manager);

    unsafe extern "C" {
        unsafe fn __restore(cx_addr: usize);
    }

    unsafe {
        __restore(KERNEL_STACK.push_context(TrapContext::app_init_context(
            APP_BASE_ADDRESS,
            USER_STACK.get_sp(),
        )) as *const _ as usize);
    }
    panic!("Unreachable in batch::run_current_app!");
}
