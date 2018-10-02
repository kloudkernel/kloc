#![feature(alloc, allocator_api, alloc_error_handler)]
#![feature(const_fn)]
#![no_std]

extern crate alloc;
extern crate linked_list_allocator;

use alloc::alloc::{Alloc, GlobalAlloc, Layout};
use alloc::prelude::ToString;
use linked_list_allocator::LockedHeap;

struct Allocator {
    inner: LockedHeap,
}

impl Allocator {
    pub unsafe fn init(&mut self, heap_start: usize, heap_end: usize) {
        let heap_size = heap_end - heap_start;
        self.inner.lock().init(heap_start, heap_size);
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        match self.inner.lock().alloc(_layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(e) => panic!(
                "failed to alloc memory for {:#?}: {}",
                _layout,
                e.to_string()
            ),
        }
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        return self
            .inner
            .lock()
            .dealloc(core::ptr::NonNull::<u8>::new_unchecked(_ptr), _layout);
    }
}

#[global_allocator]
static mut A: Allocator = Allocator {
    inner: LockedHeap::empty(),
};

pub unsafe fn init(heap_start: usize, heap_end: usize) {
    A.init(heap_start, heap_end);
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("failed to alloc memory for {:#?}", _layout);
}
