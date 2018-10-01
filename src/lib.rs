#![feature(alloc, allocator_api)]
#![feature(const_fn)]
#![no_std]

extern crate std;
extern crate alloc;
extern crate linked_list_allocator;
extern crate spin;

mod slab;

pub const MIN_SLAB_SIZE: usize = 4096;
pub const PAGE_SIZE: usize = MIN_SLAB_SIZE;
pub const NUM_OF_SLABS: usize = 8;
pub const MIN_HEAP_SIZE: usize = NUM_OF_SLABS * MIN_SLAB_SIZE;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
