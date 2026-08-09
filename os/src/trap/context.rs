use riscv::register::sstatus::{self, FS, SPP, Sstatus};

#[repr(C, align(16))]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
    pub f: [u64; 32],
    pub fcsr: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        sstatus.set_fs(FS::Dirty);

        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
            f: [0; 32],
            fcsr: 0,
        };
        cx.set_sp(sp);

        cx
    }
}
