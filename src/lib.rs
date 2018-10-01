#![feature(alloc, allocator_api)]
#![feature(const_fn)]
#![no_std]

extern crate alloc;
extern crate linked_list_allocator;
extern crate spin;

mod slab;

use slab::Slab;

pub const MIN_SLAB_SIZE: usize = 4096;
pub const PAGE_SIZE: usize = MIN_SLAB_SIZE;
pub const NUM_OF_SLABS: usize = 8;
pub const MIN_HEAP_SIZE: usize = NUM_OF_SLABS * MIN_SLAB_SIZE;

pub struct Heap {
    slab_64_bytes: Slab,
    slab_128_bytes: Slab,
    slab_256_bytes: Slab,
    slab_512_bytes: Slab,
    slab_1024_bytes: Slab,
    slab_2048_bytes: Slab,
    slab_4096_bytes: Slab,
    linked_list_allocator: linked_list_allocator::Heap,
}

impl Heap {
    pub unsafe fn new(heap_start_addr: usize, heap_size: usize) -> Heap {
        assert!(
            heap_start_addr % PAGE_SIZE == 0,
            "heap_start_addr must be page aligned"
        );

        assert!(
            heap_size >= MIN_HEAP_SIZE,
            "heap_size must be at least equal to MIN_SLAB_SIZE"
        );

        assert!(
            heap_size % MIN_HEAP_SIZE == 0,
            "heap_size must be a multple of MIN_HEAP_SIZE"
        );

        let slab_size = heap_size / NUM_OF_SLABS;
        Heap {
            slab_64_bytes: Slab::new(heap_start_addr, slab_size, 64),
            slab_128_bytes: Slab::new(heap_start_addr + slab_size, slab_size, 128),
            slab_256_bytes: Slab::new(heap_start_addr + 2 * slab_size, slab_size, 256),
            slab_512_bytes: Slab::new(heap_start_addr + 3 * slab_size, slab_size, 512),
            slab_1024_bytes: Slab::new(heap_start_addr + 4 * slab_size, slab_size, 1024),
            slab_2048_bytes: Slab::new(heap_start_addr + 5 * slab_size, slab_size, 2048),
            slab_4096_bytes: Slab::new(heap_start_addr + 6 * slab_size, slab_size, 4096),
            linked_list_allocator: linked_list_allocator::Heap::new(
                heap_start_addr + 7 * slab_size,
                slab_size,
            ),
        }
    }
}
