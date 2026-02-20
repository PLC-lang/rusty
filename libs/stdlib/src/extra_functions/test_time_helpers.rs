// Specialized MockClock implementation for u32. Mostly copied from timers::test_time_helpers,
// which in turn is mostly copied from `https://github.com/museun/mock_instant.

use std::cell::RefCell;

thread_local! {
    pub static TIME: RefCell<u32> = RefCell::new(u32::default());
}

pub fn with_time(d: impl Fn(&mut u32)) {
    TIME.with(|t| d(&mut t.borrow_mut()))
}

pub fn get_time() -> u32 {
    TIME.with(|t| *t.borrow())
}

/// A Mock clock
///
/// This uses thread local state to have a deterministic clock.
#[derive(Copy, Clone)]
pub struct MockClock;

impl std::fmt::Debug for MockClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MockClock").field("time", &Self::time()).finish()
    }
}

impl MockClock {
    /// Set the internal clock
    pub fn set_time(time: u32) {
        with_time(|t| *t = time);
    }

    /// Advance the internal clock
    pub fn advance(time: u32) {
        with_time(|t| *t += time);
    }

    /// Get the current time
    pub fn time() -> u32 {
        get_time()
    }
}

#[cfg(feature = "mock_time")]
#[no_mangle]
pub extern "C" fn __mock_time_set_u32(secs: u32) {
    MockClock::set_time(secs);
}

#[cfg(feature = "mock_time")]
#[no_mangle]
pub extern "C" fn __mock_time_advance_u32(secs: u32) {
    MockClock::advance(secs);
}

// a mock local timezone struct to use instead of chrono::Local during testing
#[derive(Copy, Clone, Debug)]
pub struct Local;

impl Local {
    pub fn now() -> DateTime {
        DateTime::now()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct DateTime(u32);

impl DateTime {
    pub fn now() -> DateTime {
        Self(MockClock::time())
    }

    pub fn num_seconds_from_midnight(&self) -> u32 {
        MockClock::time() % (3600 * 24)
    }

    // fixed offset
    pub fn nanosecond(&self) -> u32 {
        100
    }
}
