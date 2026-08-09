.altmacro

.macro SAVE_SN n
    sd s\n, (\n+2)*8(a0)
.endm

.macro LOAD_SN n
    ld s\n, (\n+2)*8(a1)
.endm

    .section .text
    .global __switch
    # __switch(
    #     current_task_cx_ptr: *mut TaskContext,
    #     next_task_cx_ptr: *const TaskContext
    # )
    #
    # pub struct TaskContext {{
    #     ra: usize,
    #     sp: usize,
    #     s: [usize; 12],
    # }}
__switch:
    sd sp, 8(a0) # sp -> current_task_cx_ptr.sp
    sd ra, 0(a0) # ra -> current_task_cx_ptr.ra
    # s0~s11 -> current_task_cx_ptr.s[0~11]
    # s0~s11 是 callee save，其他寄存器由编译器负责保存
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n + 1
    .endr

    ld ra, 0(a1) # next_task_cx_ptr.ra -> ra
    # next_task_cx_ptr.s[0~11] -> s0~s11
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n + 1
    .endr
    ld sp, 8(a1) # next_task_cx_ptr.sp -> sp

    # execute next task
    ret
