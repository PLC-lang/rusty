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

use plc_ast::ast::AstNode;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;

use crate::{
    index::Index,
    lowering::polymorphism::table::interface::helper as itable_helper,
    resolver::AnnotationMap,
    typesystem::{DataType, DataTypeInformation, StringEncoding},
};

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
    // Only POUs implement interfaces. Interface sources are checked by `validate_interface_assignment`
    // and an unresolved source is reported elsewhere.
    let Some(pou) = index.find_pou(pou_name) else {
        let source = index.get_effective_type_or_void_by_name(pou_name);
        return (!source.is_void() && !source.is_interface()).then(|| {
            Diagnostic::invalid_interface_pou_assignment(
                &get_display_name(index, source),
                interface_name,
                location.clone(),
            )
        });
    };
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
    let ancestors = source.get_interface_hierarchy(index);

    // Check if target_iface is among the source's ancestors (which includes self).
    if ancestors.iter().any(|a| a.get_name().eq_ignore_ascii_case(target_iface)) {
        return None;
    }

    Some(Diagnostic::invalid_interface_assignment(source_iface, target_iface, location.clone()))
}

/// Validates an argument or reference target that is bound in place rather than copied, such as a
/// `VAR_IN_OUT` argument. The binding cannot convert, so the types must be the same. Returns
/// `Some(Diagnostic)` if not; `binding` says why the conversion is not possible.
///
/// ```st
/// inOut(refIA);    // ok:    exactly the parameter's interface
/// inOut(instance); // error: FbA implements IA, but is not a fat pointer to IA
/// inOut(refIB);    // error: IB extends IA, but its fat pointer points to IB's itable
/// ```
pub fn validate_bound_interface_argument(
    index: &Index,
    expected: &DataType,
    received: &DataType,
    binding: &str,
    location: &SourceLocation,
) -> Option<Diagnostic> {
    if expected.get_name().eq_ignore_ascii_case(received.get_name()) {
        return None;
    }

    Some(Diagnostic::invalid_interface_binding(
        &get_display_name(index, expected),
        &get_display_name(index, received),
        binding,
        location.clone(),
    ))
}

/// Validates that an interface reference is not called directly.
///
/// ```st
/// refIA();     // error: a method name must be specified
/// refIA.foo(); // ok:    call a declared interface method instead
/// ```
pub fn validate_direct_interface_call<T: AnnotationMap>(
    annotations: &T,
    index: &Index,
    operator: &AstNode,
) -> Option<Diagnostic> {
    let operator_type = annotations.get_type_or_void(operator, index);
    operator_type.is_interface().then(|| Diagnostic::invalid_direct_interface_call(operator.location.clone()))
}

/// Returns the name of `data_type` as it is written in source. Types declared inline (an array or
/// string parameter, a `REF_TO` variable, `THIS`) only have a generated name in the index, so they
/// are spelled out from their type information instead.
fn get_display_name(index: &Index, data_type: &DataType) -> String {
    let name = data_type.get_name();
    if !(name.starts_with("__") || name.contains(".__")) {
        return name.to_string();
    }

    match data_type.get_type_information() {
        DataTypeInformation::Pointer { inner_type_name, type_safe, .. } => {
            let keyword = match data_type.get_type_information() {
                info if info.is_reference_to() => "REFERENCE TO",
                _ if *type_safe => "REF_TO",
                _ => "POINTER TO",
            };
            format!("{keyword} {}", get_inner_display_name(index, inner_type_name))
        }

        DataTypeInformation::Array { inner_type_name, dimensions, .. } => {
            let ranges = dimensions
                .iter()
                .map(|dimension| match dimension.get_range_inclusive(index) {
                    Ok(range) => format!("{}..{}", range.start(), range.end()),
                    Err(_) => "*".to_string(),
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("ARRAY[{ranges}] OF {}", get_inner_display_name(index, inner_type_name))
        }

        DataTypeInformation::String { size, encoding, .. } => {
            let keyword = match encoding {
                StringEncoding::Utf8 => "STRING",
                StringEncoding::Utf16 => "WSTRING",
            };
            // The size counts the terminating null character
            match size.as_int_value(index) {
                Ok(size) => format!("{keyword}[{}]", size - 1),
                Err(_) => keyword.to_string(),
            }
        }

        _ => name.to_string(),
    }
}

/// Returns the display name of an element or pointee type. An alias keeps its own name.
fn get_inner_display_name(index: &Index, type_name: &str) -> String {
    match index.find_type(type_name) {
        Some(data_type) => get_display_name(index, data_type),
        None => type_name.to_string(),
    }
}
