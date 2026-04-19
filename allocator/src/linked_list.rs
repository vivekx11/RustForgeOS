//! Linked list allocator implementation

use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use linked_list_allocator::LockedHeap;

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();
