//! Pipeline phase timing.
//!
//! Enabled by setting `PLC_INCR_TIMING=1` in the environment. When enabled,
//! each timed scope logs its elapsed wall-clock time to stderr on drop,
//! indented by nesting depth so re-entrant work (e.g. a participant that
//! triggers a project-wide re-index) is visible.

use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;
use std::time::Instant;

static ENABLED: AtomicBool = AtomicBool::new(false);
static INIT: Once = Once::new();

thread_local! {
    static DEPTH: Cell<usize> = const { Cell::new(0) };
}

fn enabled() -> bool {
    INIT.call_once(|| {
        let on = std::env::var("PLC_INCR_TIMING").map(|v| !v.is_empty() && v != "0").unwrap_or(false);
        ENABLED.store(on, Ordering::Relaxed);
    });
    ENABLED.load(Ordering::Relaxed)
}

/// RAII guard that records a labelled phase timing. Logs to stderr on drop
/// when `PLC_INCR_TIMING` is enabled. No-op otherwise.
pub struct PhaseTimer {
    label: String,
    start: Instant,
    depth: usize,
    active: bool,
}

impl PhaseTimer {
    pub fn new(label: impl Into<String>) -> Self {
        let active = enabled();
        let depth = if active {
            DEPTH.with(|d| {
                let cur = d.get();
                d.set(cur + 1);
                cur
            })
        } else {
            0
        };
        Self { label: label.into(), start: Instant::now(), depth, active }
    }
}

impl Drop for PhaseTimer {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        let elapsed = self.start.elapsed();
        let indent = "  ".repeat(self.depth);
        eprintln!("[plc-timing] {indent}{}: {:.3?}", self.label, elapsed);
        DEPTH.with(|d| d.set(d.get().saturating_sub(1)));
    }
}

/// Strip the leading module path and any generic-parameter suffix from a
/// `std::any::type_name` result so the emitted label is just the type's
/// short name. Examples:
///   `foo::bar::Baz`              -> `Baz`
///   `foo::Baz<std::path::PathBuf>` -> `Baz`
pub fn short_type_name(full: &'static str) -> &'static str {
    let base = match full.find('<') {
        Some(idx) => &full[..idx],
        None => full,
    };
    base.rsplit("::").next().unwrap_or(base)
}
