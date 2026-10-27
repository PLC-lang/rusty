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

use plc_ast::ast::{AstNode, AstStatement, ReferenceAccess, ReferenceExpr};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;

use crate::{
    index::{Index, VariableIndexEntry},
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

/// The parameter or reference that an argument or `REF=` right side is bound to in place.
pub enum Binding<'a> {
    /// A `VAR_IN_OUT`, `VAR_INPUT {ref}` or `REFERENCE TO` parameter of `callee`
    Reference { parameter: &'a VariableIndexEntry, callee: &'a str },

    /// A `VAR_OUTPUT` of `callee`
    Output { parameter: &'a VariableIndexEntry, callee: &'a str },

    /// The left side of `REF=`, with its name and declaration if it is a declared variable
    RefAssignment { name: Option<String>, declaration: Option<&'a VariableIndexEntry> },
}

/// Validates an argument or reference target that is bound in place rather than copied, such as a
/// `VAR_IN_OUT` argument. The binding cannot convert, so the types must be the same. Returns
/// `Some(Diagnostic)` if not, with a note at the declaration of the parameter or reference.
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
    binding: &Binding,
    argument: &AstNode,
    location: &SourceLocation,
) -> Option<Diagnostic> {
    if expected.get_name().eq_ignore_ascii_case(received.get_name()) {
        return None;
    }

    let expected_name = get_display_name(index, expected);
    let hint = match binding {
        Binding::Reference { callee, .. } if is_assignable_to_interface(index, expected, received) => {
            let argument = match get_reference_text(argument) {
                Some(name) => format!("'{name}'"),
                None => "the argument".to_string(),
            };
            Some(format!(
                "assign {argument} to a variable of type '{expected_name}' before passing it to '{callee}'"
            ))
        }
        Binding::Reference { .. } => None,
        Binding::Output { callee, .. } => {
            Some(format!("pass a variable of type '{expected_name}' to '{callee}'"))
        }
        Binding::RefAssignment { name: Some(name), .. } => {
            Some(format!("bind '{name}' to a variable of type '{expected_name}'"))
        }
        Binding::RefAssignment { name: None, .. } => {
            Some(format!("bind the left side to a variable of type '{expected_name}'"))
        }
    };

    let (subject, declaration) = match binding {
        Binding::Reference { parameter, .. } => {
            (format!("Parameter '{}' is passed by reference", parameter.get_name()), Some(*parameter))
        }
        Binding::Output { parameter, .. } => {
            (format!("Parameter '{}' is an output", parameter.get_name()), Some(*parameter))
        }
        Binding::RefAssignment { name: Some(name), declaration } => {
            (format!("'{name}' is a reference"), *declaration)
        }
        Binding::RefAssignment { name: None, declaration } => {
            ("The left side is a reference".to_string(), *declaration)
        }
    };

    let diagnostic = Diagnostic::invalid_interface_binding(
        &expected_name,
        &get_display_name(index, received),
        hint.as_deref(),
        location.clone(),
    );
    match declaration.map(|it| &it.source_location).filter(|it| !it.is_internal()) {
        Some(declaration) => Some(diagnostic.with_sub_diagnostic(Diagnostic::interface_binding_note(
            &subject,
            &expected_name,
            declaration.clone(),
        ))),
        None => Some(diagnostic),
    }
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

/// Returns whether a value of `received` can be assigned to a variable of the interface `expected`,
/// that is whether `received` implements, is or extends `expected`.
fn is_assignable_to_interface(index: &Index, expected: &DataType, received: &DataType) -> bool {
    let interface_name = expected.get_name();
    if let Some(pou) = index.find_pou(received.get_name()) {
        return itable_helper::collect_interfaces_for_pou(index, pou).contains(interface_name);
    }

    index.find_interface(received.get_name()).is_some_and(|interface| {
        interface
            .get_interface_hierarchy(index)
            .iter()
            .any(|it| it.get_name().eq_ignore_ascii_case(interface_name))
    })
}

/// Returns the source text of `node` if it names a variable, such as `a`, `holder.r`, `arr[1]` or
/// `p^`. Other expressions may have been rewritten by an earlier lowering.
pub fn get_reference_text(node: &AstNode) -> Option<String> {
    let mut current = node;
    loop {
        match &current.stmt {
            AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Member(_), base: None })
            | AstStatement::This
            | AstStatement::Super(_) => {
                return Some(node.as_string());
            }
            AstStatement::ReferenceExpr(ReferenceExpr {
                access: ReferenceAccess::Member(_) | ReferenceAccess::Index(_) | ReferenceAccess::Deref,
                base: Some(base),
            }) => {
                current = base;
            }
            _ => return None,
        }
    }
}
