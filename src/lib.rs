#![doc=include_str!("../README.md")]
#![no_std]

mod pool;
mod string_id;

pub use crate::pool::{Pool, PoolAllocator};
pub use crate::string_id::StringId;
