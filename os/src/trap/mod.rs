use core::arch::{asm, global_asm};

use riscv::{
    interrupt::{Exception, Interrupt, Trap},
    register::{
        scause, stval,
        stvec::{self, TrapMode},
    },
};

use crate::{
    config::{TRAMPLINE, TRAP_CONTEXT},
    sbi::shutdown,
    syscall::syscall,
    task::{
        current_trap_cx, current_user_token, exit_current_and_run_next,
        suspend_current_and_run_next,
    },
    time::timer::set_next_trigger,
};

mod context;
pub use context::TrapContext;

global_asm!(include_str!("trap.S"));

pub fn init() {
    unsafe extern "C" {
        unsafe fn __alltraps();
    }
    unsafe {
        stvec::write(stvec::Stvec::new(
            linker_symbol_addr!(__alltraps),
            TrapMode::Direct,
        ));
    }
}

pub fn enable_timer_interrupt() {
    unsafe {
        riscv::register::sie::set_stimer();
    }
}

#[unsafe(no_mangle)]
pub fn trap_from_kernel() -> ! {
    panic!("a trap from kernel!");
}

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(stvec::Stvec::new(
            linker_symbol_addr!(trap_from_kernel),
            TrapMode::Direct,
        ))
    }
}

fn set_user_trap_entry() {
    unsafe { stvec::write(stvec::Stvec::new(TRAMPLINE, TrapMode::Direct)) }
}

#[unsafe(no_mangle)]
pub fn trap_return() -> ! {
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp = current_user_token();

    unsafe extern "C" {
        unsafe fn __alltraps();
        unsafe fn __restore();
    }

    let restore_va = TRAMPLINE + linker_symbol_addr!(__restore) - linker_symbol_addr!(__alltraps);
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,
            in("a1") user_satp,
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
pub fn trap_handler() -> ! {
    set_kernel_trap_entry();

    let cx = current_trap_cx();
    let scause = scause::read();
    let stval = stval::read();

    let trap: Trap<Interrupt, Exception> = match scause.cause().try_into() {
        Ok(trap) => trap,
        Err(_) => panic!(
            "Unsupported trap {:?}, stval = {:#x}!",
            scause.cause(),
            stval
        ),
    };

    match trap {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(
            fault @ (Exception::InstructionFault
            | Exception::LoadFault
            | Exception::StoreFault
            | Exception::InstructionPageFault
            | Exception::LoadPageFault
            | Exception::StorePageFault),
        ) => {
            log::info!(
                "[kernel] {:?} in application: sepc={:#x}, stval={:#x}; kernel killed it.",
                fault,
                cx.sepc,
                stval
            );
            exit_current_and_run_next();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            log::info!(
                "[kernel] IllegalInstruction in application: sepc={:#x}; kernel killed it.",
                cx.sepc
            );
            exit_current_and_run_next();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            log::error!("Unsupported trap {:?}, stval = {:#x}!", trap, stval);
            shutdown(true);
        }
    }

    trap_return();
}
