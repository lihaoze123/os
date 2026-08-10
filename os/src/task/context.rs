use crate::trap::trap_return;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub fn goto_trap_return(kernel_stack_top: usize) -> Self {
        Self {
            ra: linker_symbol_addr!(trap_return),
            sp: kernel_stack_top,
            s: [0; 12],
        }
    }
}
