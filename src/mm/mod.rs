pub mod vm;

pub mod allocator;

pub use allocator::{alloc, dealloc, buddy_allocator, simple_allocator, change_to_buddy};
