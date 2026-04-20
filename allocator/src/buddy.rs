//! Buddy allocator implementation
//! A buddy allocator divides memory into power-of-2 sized blocks.
//! When a block is freed, it's merged with its "buddy" if the buddy is also free.

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::{self, NonNull};
use spin::Mutex;

const MIN_BLOCK_SIZE: usize = 16;
const MAX_ORDER: usize = 20; // 2^20 = 1MB max block

struct BuddyAllocator {
    heap_start: usize,
    heap_size: usize,
    free_lists: [Option<NonNull<FreeBlock>>; MAX_ORDER],
}

#[repr(C)]
struct FreeBlock {
    next: Option<NonNull<FreeBlock>>,
}

impl BuddyAllocator {
    pub const fn new() -> Self {
        const INIT: Option<NonNull<FreeBlock>> = None;
        Self {
            heap_start: 0,
            heap_size: 0,
            free_lists: [INIT; MAX_ORDER],
        }
    }

    /// Initialize the buddy allocator
    ///
    /// # Safety
    /// Must be called exactly once with valid heap bounds
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_size = heap_size;

        // Add the entire heap as one large block
        let order = self.size_to_order(heap_size);
        let block = heap_start as *mut FreeBlock;
        (*block).next = None;
        self.free_lists[order] = NonNull::new(block);
    }

    fn size_to_order(&self, size: usize) -> usize {
        let size = size.max(MIN_BLOCK_SIZE);
        let order = (size.next_power_of_two().trailing_zeros() as usize)
            .saturating_sub(MIN_BLOCK_SIZE.trailing_zeros() as usize);
        order.min(MAX_ORDER - 1)
    }

    fn order_to_size(&self, order: usize) -> usize {
        MIN_BLOCK_SIZE << order
    }

    unsafe fn allocate(&mut self, layout: Layout) -> *mut u8 {
        let size = layout.size().max(layout.align());
        let order = self.size_to_order(size);

        // Find a free block of sufficient size
        for current_order in order..MAX_ORDER {
            if let Some(mut block) = self.free_lists[current_order] {
                // Remove from free list
                self.free_lists[current_order] = unsafe { block.as_ref().next };

                // Split larger blocks down to required size
                for split_order in (order..current_order).rev() {
                    let buddy_offset = self.order_to_size(split_order);
                    let buddy = (block.as_ptr() as usize + buddy_offset) as *mut FreeBlock;
                    
                    unsafe {
                        (*buddy).next = self.free_lists[split_order];
                        self.free_lists[split_order] = NonNull::new(buddy);
                    }
                }

                return block.as_ptr() as *mut u8;
            }
        }

        ptr::null_mut()
    }

    unsafe fn deallocate(&mut self, ptr: *mut u8, layout: Layout) {
        let size = layout.size().max(layout.align());
        let mut order = self.size_to_order(size);
        let mut block_addr = ptr as usize;

        // Try to merge with buddy blocks
        while order < MAX_ORDER - 1 {
            let block_size = self.order_to_size(order);
            let buddy_addr = block_addr ^ block_size;

            // Check if buddy is in our heap
            if buddy_addr < self.heap_start || buddy_addr >= self.heap_start + self.heap_size {
                break;
            }

            // Try to find and remove buddy from free list
            if !self.remove_from_free_list(order, buddy_addr) {
                break;
            }

            // Merge with buddy
            block_addr = block_addr.min(buddy_addr);
            order += 1;
        }

        // Add merged block to free list
        let block = block_addr as *mut FreeBlock;
        unsafe {
            (*block).next = self.free_lists[order];
            self.free_lists[order] = NonNull::new(block);
        }
    }

    fn remove_from_free_list(&mut self, order: usize, addr: usize) -> bool {
        let mut current = &mut self.free_lists[order];

        while let Some(mut node) = *current {
            if node.as_ptr() as usize == addr {
                *current = unsafe { node.as_ref().next };
                return true;
            }
            current = unsafe { &mut node.as_mut().next };
        }

        false
    }
}

pub struct LockedBuddyAllocator(Mutex<BuddyAllocator>);

impl LockedBuddyAllocator {
    pub const fn new() -> Self {
        Self(Mutex::new(BuddyAllocator::new()))
    }

    pub unsafe fn init(&self, heap_start: usize, heap_size: usize) {
        self.0.lock().init(heap_start, heap_size);
    }
}

unsafe impl GlobalAlloc for LockedBuddyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.lock().allocate(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.lock().deallocate(ptr, layout);
    }
}

#[cfg(feature = "buddy")]
#[global_allocator]
pub static ALLOCATOR: LockedBuddyAllocator = LockedBuddyAllocator::new();
