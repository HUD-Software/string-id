use super::{Slot, PoolAllocator};

#[derive(Copy, Clone)]
pub struct Slots<'alloc, SlotAllocator: PoolAllocator> {
    slots: *mut Slot,
    allocator: &'alloc SlotAllocator,
}

impl<'alloc, SlotAllocator: PoolAllocator> Slots<'alloc, SlotAllocator> 
 where Slots<'alloc, SlotAllocator>: core::marker::Copy{
    pub fn new_in(allocator: &'alloc SlotAllocator) -> Slots<'alloc, SlotAllocator> {
        Slots {
            slots: core::ptr::null_mut::<Slot>(),
            allocator,
        }
    }
}
