//! Cooperative cancellation primitive shared by the compile pipeline
//! and any consumer that runs work it might need to abandon (today: the
//! LSP server's compile worker).
//!
//! The token is an `Arc<AtomicBool>`. Anyone with a clone can either
//! observe the cancelled state (`is_cancelled`) or flip it
//! (`cancel`). Once cancelled, it stays cancelled — there's no
//! "reset" operation, by design. The compile pipeline reads the flag
//! at known check points (per-unit boundaries in parse/index/annotate,
//! before each validator pass, at the top of each lowering
//! participant) and returns an `Err(Diagnostic::cancelled())` to
//! short-circuit. Callers post-check the token to classify the `Err`
//! as cancellation vs. a real failure.
//!
//! The `Default` impl returns a never-cancelled token so callers that
//! don't care about cancellation (CLI, tests) get the no-op for free.
//!
//! See `.baseline/lsp-decisions-log.md` Q1/Q4 in the cancellation
//! design and the phase-6 plan for the surrounding rationale.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::diagnostics::Diagnostic;

/// Error code stamped on `Diagnostic::cancelled()`. Used by callers
/// that prefer to distinguish cancellation by inspecting the error
/// itself rather than re-checking the token.
pub const CANCELLED_ERROR_CODE: &str = "CANCELLED";

/// Shared cancellation flag. Clone is cheap (it's an `Arc<AtomicBool>`
/// clone); all clones observe the same underlying flag.
#[derive(Default, Clone, Debug)]
pub struct CancellationToken(Arc<AtomicBool>);

impl CancellationToken {
    pub fn new() -> Self {
        Self::default()
    }

    /// Has anyone called `cancel` on this token (or a clone)?
    pub fn is_cancelled(&self) -> bool {
        // Relaxed is sufficient: we're using this for cooperative
        // cancellation, not lock-free synchronisation. A delayed
        // observation just means one more unit's worth of work before
        // we notice.
        self.0.load(Ordering::Relaxed)
    }

    /// Flip the flag to "cancelled". Idempotent.
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Relaxed);
    }

    /// Cooperative check at a pipeline boundary. Returns
    /// `Err(Diagnostic::cancelled())` if cancelled, `Ok(())` otherwise.
    /// Designed for the `cancellation.check()?` call-site idiom.
    pub fn check(&self) -> Result<(), Diagnostic> {
        if self.is_cancelled() {
            Err(Diagnostic::cancelled())
        } else {
            Ok(())
        }
    }
}

impl Diagnostic {
    /// Construct the sentinel diagnostic used to short-circuit a
    /// pipeline stage on cancellation. The diagnostic is never
    /// published to the LSP client — the compile worker intercepts it
    /// and emits `CompileOutcome::Cancelled` instead. Carries
    /// `error_code = CANCELLED_ERROR_CODE` so callers that prefer to
    /// inspect the error rather than re-check the token can do so.
    ///
    /// Sets the `error_code` field directly rather than going through
    /// `with_error_code` because that builder validates against the
    /// `DIAGNOSTICS` registry, which doesn't list our cancellation
    /// sentinel (it's intentionally not a user-facing error code).
    pub fn cancelled() -> Self {
        let mut d = Diagnostic::new("compile cancelled");
        d.inner.error_code = CANCELLED_ERROR_CODE;
        d
    }

    /// True iff this diagnostic was produced by `cancelled()` — i.e.
    /// its error code matches `CANCELLED_ERROR_CODE`. Lets call sites
    /// classify a pipeline `Err` without re-reading the token (handy
    /// in places that don't have the token in scope).
    pub fn is_cancelled(&self) -> bool {
        self.error_code == CANCELLED_ERROR_CODE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_token_is_not_cancelled() {
        let t = CancellationToken::new();
        assert!(!t.is_cancelled());
    }

    #[test]
    fn cancel_flips_the_flag() {
        let t = CancellationToken::new();
        t.cancel();
        assert!(t.is_cancelled());
    }

    #[test]
    fn cancel_is_idempotent() {
        let t = CancellationToken::new();
        t.cancel();
        t.cancel();
        assert!(t.is_cancelled());
    }

    #[test]
    fn clones_share_state() {
        let t1 = CancellationToken::new();
        let t2 = t1.clone();
        t1.cancel();
        // The clone observes the cancellation — they're not independent.
        assert!(t2.is_cancelled());
    }

    #[test]
    fn check_returns_ok_when_not_cancelled() {
        let t = CancellationToken::new();
        assert!(t.check().is_ok());
    }

    #[test]
    fn check_returns_cancelled_diagnostic_when_cancelled() {
        let t = CancellationToken::new();
        t.cancel();
        let err = t.check().expect_err("should be cancelled");
        assert_eq!(err.get_error_code(), CANCELLED_ERROR_CODE);
        assert!(err.is_cancelled());
    }

    #[test]
    fn default_token_is_not_cancelled() {
        // The Default impl is what CLI callers see when they don't
        // care about cancellation. Must be the no-op flavour.
        let t = CancellationToken::default();
        assert!(!t.is_cancelled());
        assert!(t.check().is_ok());
    }

    #[test]
    fn diagnostic_cancelled_constructor() {
        let d = Diagnostic::cancelled();
        assert_eq!(d.get_error_code(), CANCELLED_ERROR_CODE);
        assert!(d.is_cancelled());
    }

    #[test]
    fn diagnostic_is_cancelled_false_for_regular_diagnostic() {
        let d = Diagnostic::new("regular error");
        assert!(!d.is_cancelled());
    }
}
