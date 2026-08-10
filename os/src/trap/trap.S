.altmacro
.option push
.option arch, +d

.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm

.macro SAVE_FP n
    fsd f\n, (34+\n)*8(sp)
.endm

    .section .text.trampoline
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
    csrrw sp, sscratch, sp
    # now sp->*TrapContext in user space, sscratch->user stack
    # save general-purpose registers
    sd x1, 1*8(sp)
    # skip sp(x2), we will save it later
    sd x3, 3*8(sp)
    # skip tp(x4), application does not use it
    # save x5~x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # we can use t0/t1/t2 freely, because they were saved on kernel stack
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # 浮点寄存器
    .set n, 0
    .rept 32
        SAVE_FP %n
        .set n, n+1
    .endr
    csrr t0, fcsr
    sd t0, 66*8(sp)
    # read user stack from sscratch and save it on the kernel stack
    csrr t2, sscratch
    sd t2, 2*8(sp)
    # load kernel_satp, kernel_sp and trap_handler
    ld t0, 67*8(sp)
    ld t1, 68*8(sp)
    ld t2, 69*8(sp)
    # switch to the kernel address space and kernel stack
    csrw satp, t0
    sfence.vma
    mv sp, t1
    # jump to trap_handler
    jr t2

.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

.macro LOAD_FP n
    fld f\n, (34+\n)*8(sp)
.endm

__restore:
    # a0: *TrapContext in user space(Constant)
    # a1: user space token
    # switch to user space
    csrw satp, a1
    sfence.vma
    csrw sscratch, a0
    mv sp, a0
    # sp points to TrapContext in user space, start restoring based on it
    # restore sstatus/sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    # restore floating-point state before t0 is restored below
    .set n, 0
    .rept 32
        LOAD_FP %n
        .set n, n+1
    .endr
    ld t0, 66*8(sp)
    csrw fcsr, t0
    # restore general-purpuse registers except sp/tp
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # restore the user stack pointer last, while sp still points to TrapContext
    ld sp, 2*8(sp)
    sret

.option pop
