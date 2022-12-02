use super::{PoolAllocator, Slot};

pub struct Slots<'alloc, SlotAllocator: PoolAllocator> {
    slots: *mut Slot,
    allocator: &'alloc SlotAllocator,
}

impl<'alloc, SlotAllocator: PoolAllocator> Copy for Slots<'alloc, SlotAllocator> {}

impl<'alloc, SlotAllocator: PoolAllocator> Clone for Slots<'alloc, SlotAllocator> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'alloc, SlotAllocator: PoolAllocator> Slots<'alloc, SlotAllocator> {
    pub fn new_in(allocator: &'alloc SlotAllocator) -> Slots<'alloc, SlotAllocator> {
        Slots {
            slots: core::ptr::null_mut::<Slot>(),
            allocator,
        }
    }
}
