#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub fn goto_restore(app_cx: usize) -> Self {
        unsafe extern "C" {
            unsafe fn __restore();
        }

        Self {
            ra: linker_symbol_addr!(__restore),
            sp: app_cx,
            s: [0; 12],
        }
    }
}
