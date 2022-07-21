use super::Slot;
use core::alloc::Layout;
pub struct Slots {
    slots: *mut Slot,
}