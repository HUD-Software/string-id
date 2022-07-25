mod slot;
mod slots;
use slot::Slot;
use slots::Slots;

use core::ptr::NonNull;

pub unsafe trait PoolAllocator {
    fn allocate(&self, _:core::alloc::Layout) ->  Result<NonNull<[u8]>, core::alloc::AllocError>;
}

pub struct Pool<'alloc, SlotAllocator>
where
    SlotAllocator: PoolAllocator,
{
    slots: [Slots<'alloc, SlotAllocator>; 256],
}


impl<'alloc, SlotAllocator> Pool<'alloc, SlotAllocator>
where
    SlotAllocator: PoolAllocator,
    Slots<'alloc, SlotAllocator>: core::marker::Copy
{
    pub fn new(allocator: &'alloc SlotAllocator) -> Self {
        Pool {
            slots: [Slots::<'alloc, SlotAllocator>::new_in(allocator); 256],
        }
    }
}
