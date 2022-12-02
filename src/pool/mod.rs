mod slot;
mod slots;
use slot::Slot;
use slots::Slots;

pub unsafe trait PoolAllocator {
    fn allocate(&self, _: core::alloc::Layout) -> *mut u8;
}

pub struct Pool<'alloc, SlotAllocator: PoolAllocator> {
    slots: [Slots<'alloc, SlotAllocator>; 256],
}

impl<'alloc, SlotAllocator: PoolAllocator> Pool<'alloc, SlotAllocator> {
    pub fn new(allocator: &'alloc SlotAllocator) -> Self {
        Pool {
            slots: [Slots::<'alloc, SlotAllocator>::new_in(allocator); 256],
        }
    }
}
