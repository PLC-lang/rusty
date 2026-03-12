//! Validation functions for interface polymorphism, called during lowering.
//!
//! Each function returns `Some(Diagnostic)` when the code is invalid, or `None` when valid.
//!
//! Note:
//! These checks run inside [`super::interface::InterfaceDispatchLowerer`] rather than in the
//! normal validation pass because the lowerer rewrites interface types to `__FATPOINTER` before
//! the validator sees them. Running these checks here ensures error messages reference the
//! user-written interface names, not internal types.
//!
//! Technically a `pre_generate` exists, which would mean polymorphic lowering happens after validation but
//! other lowerers would then also need to change hooking from `post_annotate` to `pre_generate` and also
//! break existing validations (some SUPER tests failed when I tried this). As a result, doing validations
//! during lowering is the most pragmatic option right now, even if it's a bit inelegant.

use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;

use crate::index::Index;
use crate::lowering::polymorphism::table::interface::helper as itable_helper;

/// Validates that a POU implements the target interface. Returns `Some(Diagnostic)` if not.
///
/// ```st
/// refIA := instanceFbX;  // error: FbX doesn't implement IA
/// refIA := instanceFbA;  // ok:    FbA IMPLEMENTS IA
/// refIA := instanceFbB;  // ok:    FbB EXTENDS FbA, FbA IMPLEMENTS IA
/// ```
pub fn validate_pou_implements_interface(
    index: &Index,
    pou_name: &str,
    interface_name: &str,
    location: &SourceLocation,
) -> Option<Diagnostic> {
    let pou = index.find_pou(pou_name)?;
    let all_interfaces = itable_helper::collect_interfaces_for_pou(index, pou);

    if all_interfaces.contains(interface_name) {
        return None;
    }

    Some(Diagnostic::invalid_interface_pou_assignment(pou_name, interface_name, location.clone()))
}

/// Validates that an interface-to-interface assignment is a valid upcast. Returns `Some(Diagnostic)` if not.
///
/// ```st
/// refIA := refIB;  // ok:    IB EXTENDS IA
/// refIB := refIA;  // error: downcast
/// refIA := refIC;  // error: IC unrelated to IA
/// ```
pub fn validate_interface_assignment(
    index: &Index,
    source_iface: &str,
    target_iface: &str,
    location: &SourceLocation,
) -> Option<Diagnostic> {
    // Same type is always valid (handled before we get here, but be defensive).
    if source_iface.eq_ignore_ascii_case(target_iface) {
        return None;
    }

    let source = index.find_interface(source_iface)?;
    let ancestors = source.get_parent_interfaces_recursive(index);

    // Check if target_iface is among the source's ancestors (which includes self).
    if ancestors.iter().any(|a| a.get_name().eq_ignore_ascii_case(target_iface)) {
        return None;
    }

    Some(Diagnostic::invalid_interface_assignment(source_iface, target_iface, location.clone()))
}
