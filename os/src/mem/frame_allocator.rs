use alloc::vec::Vec;
use lazy_static::lazy_static;

use crate::{
    config::MEMORY_END,
    mem::address::{PhysAddr, PhysPageNum},
    sync::up::UPSafeCell,
};

trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum);
}

#[derive(Default)]
pub struct StackFrameAllocator {
    current: usize,
    end: usize,
    recycled: Vec<usize>,
}

impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        Self::default()
    }

    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop() {
            Some(ppn.into())
        } else {
            if self.current == self.end {
                None
            } else {
                self.current += 1;
                Some((self.current - 1).into())
            }
        }
    }

    fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn = ppn.0;
        if ppn >= self.current || self.recycled.iter().find(|&&v| v == ppn).is_some() {
            panic!("Frame ppn={:#x} has not been allocated!", ppn);
        }
        self.recycled.push(ppn);
    }
}

impl StackFrameAllocator {
    pub fn init(&mut self, l: PhysPageNum, r: PhysPageNum) {
        self.current = l.0;
        self.end = r.0;
    }
}

type FrameAllocatorImpl = StackFrameAllocator;

lazy_static! {
    pub static ref FRAME_ALLOCATOR: UPSafeCell<FrameAllocatorImpl> =
        unsafe { UPSafeCell::new(FrameAllocatorImpl::new()) };
}

pub fn init_frame_allocator() {
    unsafe extern "C" {
        safe fn ekernel();
    }
    let l = PhysAddr::from(linker_symbol_addr!(ekernel)).ceil();
    let r = PhysAddr::from(MEMORY_END).floor();
    FRAME_ALLOCATOR.borrow_mut().init(l, r);
}

pub fn frame_alloc() -> Option<FrameTracker> {
    FRAME_ALLOCATOR
        .borrow_mut()
        .alloc()
        .map(FrameTracker::new)
}

fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR.borrow_mut().dealloc(ppn);
}

pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

impl FrameTracker {
    pub fn new(ppn: PhysPageNum) -> Self {
        ppn.get_bytes_array().fill(0);
        Self { ppn }
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);
    }
}
