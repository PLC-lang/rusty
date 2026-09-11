//! A global allocator for the test binary that counts allocations per thread, so tests can
//! assert that a runtime path does not touch the heap.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

thread_local! {
    static ALLOCATIONS: Cell<usize> = const { Cell::new(0) };
}

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let _ = ALLOCATIONS.try_with(|count| count.set(count.get() + 1));
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let _ = ALLOCATIONS.try_with(|count| count.set(count.get() + 1));
        System.realloc(ptr, layout, new_size)
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

/// The number of heap allocations made on the current thread so far.
pub fn allocations() -> usize {
    ALLOCATIONS.with(Cell::get)
}

/// Runs `f` and returns how many heap allocations it made on the current thread.
pub fn allocations_during(f: impl FnOnce()) -> usize {
    let before = allocations();
    f();
    allocations() - before
}

#[cfg(test)]
mod tests {
    use super::allocations_during;

    #[test]
    fn counts_heap_allocations_on_this_thread() {
        assert_eq!(allocations_during(|| {}), 0);
        assert!(allocations_during(|| std::hint::black_box(String::from("allocates")).clear()) >= 1);
    }
}
