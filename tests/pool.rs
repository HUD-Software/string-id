

#![feature(allocator_api)]
use string_id::{Pool, PoolAllocator};

struct MyAllocator{

}

unsafe impl PoolAllocator for MyAllocator{
    fn allocate(&self, layout: Layout) -> Result<core::ptr::NonNull<[u8]>, core::alloc::AllocError> { 
        let ptr = libc::malloc
    }
}

#[test]
fn test_pool() {
    //  let pool = Pool::new_in()
}
