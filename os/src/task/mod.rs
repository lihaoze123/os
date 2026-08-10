use crate::{
    config::{TRAP_CONTEXT, kernel_stack_position},
    mem::{
        KERNEL_SPACE,
        address::{PhysPageNum, VirtAddr},
        memory_set::{MapPermission, MemorySet},
    },
    task::context::TaskContext,
    trap::{TrapContext, trap_handler},
};

use self::task_manager::TASK_MANAGER;

mod context;
mod switch;
mod task_manager;

#[derive(Clone, Copy, PartialEq)]
pub enum TaskStatus {
    Ready,
    Running,
    Exited,
}

pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub memory_set: MemorySet,
    pub trap_cx_ppn: PhysPageNum,
}

impl TaskControlBlock {
    pub fn new(elf_data: &[u8], app_id: usize) -> Self {
        let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
        let trap_cx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        let task_status = TaskStatus::Ready;

        let (kernel_stack_bottom, kernel_stack_top) = kernel_stack_position(app_id);
        KERNEL_SPACE.borrow_mut().insert_framed_area(
            kernel_stack_bottom.into(),
            kernel_stack_top.into(),
            MapPermission::R | MapPermission::W,
        );

        let task_control_block = Self {
            task_status,
            task_cx: TaskContext::goto_trap_return(kernel_stack_top),
            memory_set,
            trap_cx_ppn,
        };

        let trap_cx = task_control_block.get_trap_cx();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.borrow_mut().token(),
            kernel_stack_top,
            linker_symbol_addr!(trap_handler),
        );

        task_control_block
    }

    fn get_trap_cx(&self) -> &'static mut TrapContext {
        let trap_cx: &mut TrapContext = self.trap_cx_ppn.get_mut();
        trap_cx
    }

    pub fn get_user_token(&self) -> usize {
        self.memory_set.token()
    }
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

pub fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

pub fn current_user_token() -> usize {
    TASK_MANAGER.get_current_token()
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    TASK_MANAGER.get_current_trap_cx()
}
