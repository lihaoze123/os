use core::{
    iter::FlatMap,
    slice::{Iter, IterMut},
};

use alloc::vec;
use alloc::vec::Vec;

use crate::mem::{
    address::{StepByOne, VirtAddr},
    page_table::PageTable,
};

pub struct UserBuffer {
    buffers: Vec<&'static mut [u8]>,
    len: usize,
}

impl UserBuffer {
    pub fn new(token: usize, ptr: *const u8, len: usize) -> Self {
        let page_table = PageTable::from_token(token);
        let mut start = ptr as usize;
        let end = start + len;
        let mut buffers = Vec::new();

        while start < end {
            let start_va = VirtAddr::from(start);
            let mut vpn = start_va.floor();
            let ppn = page_table.translate(vpn).unwrap().ppn();
            vpn.step();
            let mut end_va = VirtAddr::from(vpn);
            end_va = end_va.min(VirtAddr::from(end));
            if end_va.page_offset() == 0 {
                buffers.push(&mut ppn.get_bytes_array()[start_va.page_offset()..]);
            } else {
                buffers
                    .push(&mut ppn.get_bytes_array()[start_va.page_offset()..end_va.page_offset()]);
            }
            start = end_va.into();
        }

        Self { buffers, len }
    }

    pub fn iter(&self) -> UserBufferIter<'_> {
        self.buffers.iter().flat_map(|buf| buf.iter())
    }

    pub fn iter_mut(&mut self) -> UserBufferIterMut<'_> {
        self.buffers.iter_mut().flat_map(|buf| buf.iter_mut())
    }

    pub fn write(&mut self, src: &[u8]) {
        for (i, dst) in self.iter_mut().enumerate() {
            *dst = src[i];
        }
    }

    pub unsafe fn write_raw_value_unchecked<T>(&mut self, src: T) {
        let t_bytes = unsafe {
            let t_ptr = &src as *const T as *const u8;
            core::slice::from_raw_parts(t_ptr, size_of::<T>())
        };
        self.write(t_bytes)
    }

    pub fn read(&self, dst: &mut [u8]) {
        for (i, src) in self.iter().enumerate() {
            dst[i] = *src;
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![0; self.len];
        self.read(&mut buf);
        buf
    }
}

type UserBufferIter<'a> =
    FlatMap<Iter<'a, &'static mut [u8]>, Iter<'a, u8>, fn(&'a &'static mut [u8]) -> Iter<'a, u8>>;

type UserBufferIterMut<'a> = FlatMap<
    IterMut<'a, &'static mut [u8]>,
    IterMut<'a, u8>,
    fn(&'a mut &'static mut [u8]) -> IterMut<'a, u8>,
>;
