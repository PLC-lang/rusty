// This module is mostly copied from `https://github.com/museun/mock_instant`
// The reason is that the repository has not been updated in a while.

use std::{cell::RefCell, time::Duration};

thread_local! {
    pub static TIME: RefCell<Duration> = RefCell::new(Duration::default());
}

pub fn with_time(d: impl Fn(&mut Duration)) {
    TIME.with(|t| d(&mut t.borrow_mut()))
}

pub fn get_time() -> Duration {
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
    /// Set the internal clock to this 'Duration'
    pub fn set_time(time: Duration) {
        with_time(|t| *t = time);
    }

    /// Advance the internal clock by this 'Duration'
    pub fn advance(time: Duration) {
        with_time(|t| *t += time);
    }

    /// Get the current duration
    pub fn time() -> Duration {
        get_time()
    }
}

/// A simple deterministic Instant wrapped around a modifiable Duration
///
/// This used a thread-local state as the 'wall clock' that is configurable via
/// the `MockClock`
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Instant(Duration);

impl Instant {
    pub fn now() -> Instant {
        Self(MockClock::time())
    }

    pub fn elapsed(&self) -> Duration {
        MockClock::time() - self.0
    }
}
