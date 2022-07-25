#![feature(allocator_api)]
use std::alloc::{alloc, Layout};
use string_id::{Pool, PoolAllocator};

struct MyAllocator{

}

unsafe impl PoolAllocator for MyAllocator{
    fn allocate(&self, layout: Layout) -> * mut u8 { 
        unsafe { alloc(layout) }
    }
}

#[test]
fn test_pool() {
    //  let pool = Pool::new_in()
}
