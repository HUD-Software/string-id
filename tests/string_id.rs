use std::alloc::{alloc, Layout};
use string_id::{Pool, PoolAllocator, StringId};

struct MyAllocator;

unsafe impl PoolAllocator for MyAllocator {
    fn allocate(&self, layout: Layout) -> *mut u8 {
        unsafe { alloc(layout) }
    }
}

#[test]
fn test_creation() {
    let id = StringId::from("none");
}
