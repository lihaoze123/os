use core::arch::global_asm;

use riscv::{
    interrupt::{Exception, Interrupt, Trap},
    register::{
        scause, stval,
        stvec::{self, TrapMode},
    },
};

use crate::{sbi::shutdown, syscall::syscall, task::run_next_task};

mod context;
pub use context::TrapContext;

global_asm!(include_str!("trap.S"));

pub fn init() {
    unsafe extern "C" {
        safe fn __alltraps();
    }
    unsafe {
        stvec::write(stvec::Stvec::new(
            linker_symbol_addr!(__alltraps),
            TrapMode::Direct,
        ));
    }
}

#[unsafe(no_mangle)]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
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
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            log::info!("[kernel] PageFault in application, kernel killed it.");
            run_next_task();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            log::info!("[kernel] IllegalInstruction in application, kernel killed it.");
            run_next_task();
        }
        Trap::Exception(Exception::InstructionFault) => {
            log::info!("[kernel] InstructionFault in application, kernel killed it.");
            run_next_task();
        }
        Trap::Exception(Exception::LoadFault) => {
            log::info!("[kernel] InstructionFault in application, kernel killed it.");
            run_next_task();
        }
        _ => {
            log::error!("Unsupported trap {:?}, stval = {:#x}!", trap, stval);
            shutdown(true);
        }
    }

    cx
}
