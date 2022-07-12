use string_id_page_allocator::StringIdPageAllocator;

struct StringIdPool<AllocatorT>{
    allocator: StringIdPageAllocator<T>,
}


impl<T> StringIDPool<T> {
    pub fn new() -> Self {
        StringIdPool::<T> {
            allocator: StringIdPageAllocator::<T>{}, 
        }
    }
}