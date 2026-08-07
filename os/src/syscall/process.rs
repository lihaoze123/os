use crate::batch::{get_current_task_info, run_next_app};

pub fn sys_get_taskinfo() -> isize {
    let (task_id, task_name) = get_current_task_info();
    log::info!("[kernel] task id: {}, task name: {}", task_id, task_name);
    task_id as isize
}

pub fn sys_exit(xstate: i32) -> ! {
    log::info!("[kernel] Application exited with code {}", xstate);
    run_next_app()
}
