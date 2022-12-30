pub unsafe trait PoolAllocator {
    fn allocate(&self, _: core::alloc::Layout) -> *mut u8;
}

