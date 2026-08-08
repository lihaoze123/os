use crate::{
    loader::get_num_app,
    sbi::shutdown,
    sync::up::UPSafeCell,
    task::{TaskControlBlock, TaskStatus, context::TaskContext, init_app_cx, switch::__switch},
};

pub(super) const MAX_APP_NUM: usize = 16;

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

impl TaskManager {
    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.borrow_mut();
        let current = inner.current_task;

        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|&id| inner.tasks[id].task_status == TaskStatus::Ready)
    }

    pub fn run_first_task(&self) {
        let mut inner = self.inner.borrow_mut();
        if self.num_app > 0 && inner.tasks[0].task_status == TaskStatus::Ready {
            inner.tasks[0].task_status = TaskStatus::Running;
            inner.current_task = 0;

            let temp_cx_ptr = &mut TaskContext::default() as *mut _;
            let next_task_cx_ptr = &inner.tasks[0].task_cx as *const TaskContext;

            drop(inner);
            unsafe {
                __switch(temp_cx_ptr, next_task_cx_ptr);
            }
        } else {
            log::warn!("there are no applications to run");
            shutdown(false)
        }
    }

    pub fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.borrow_mut();
            let current = inner.current_task;

            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;

            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;

            drop(inner);
            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            log::info!("All applications completed!");
            shutdown(false)
        }
    }

    pub fn mark_current_suspended(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    pub fn mark_current_exited(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }
}

pub(super) struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

lazy_static::lazy_static! {
    pub(super) static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [
            TaskControlBlock {
                task_cx: TaskContext::default(),
                task_status: TaskStatus::UnInit,
            };
            MAX_APP_NUM
        ];
        tasks.iter_mut().enumerate().for_each(|(i, task)| {
            task.task_cx = TaskContext::goto_restore(init_app_cx(i));
            task.task_status = TaskStatus::Ready;
        });

        TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner { tasks, current_task: 0 })
            }
        }
    };
}
