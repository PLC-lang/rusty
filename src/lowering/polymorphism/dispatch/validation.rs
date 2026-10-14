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
    let ancestors = source.get_interface_hierarchy(index);

    // Check if target_iface is among the source's ancestors (which includes self).
    if ancestors.iter().any(|a| a.get_name().eq_ignore_ascii_case(target_iface)) {
        return None;
    }

    Some(Diagnostic::invalid_interface_assignment(source_iface, target_iface, location.clone()))
}

/// Validates that an interface variable passed to a by-reference parameter (`VAR_OUTPUT`, `VAR_IN_OUT`,
/// `VAR_INPUT {ref}` or `REFERENCE TO`) has exactly the parameter's interface type. The callee writes through the parameter, so
/// an upcast into a temporary fat pointer would never reach the caller. Returns `Some(Diagnostic)` if not.
///
/// ```st
/// consumer(inout1 := refIA);  // ok:    inout1 : IA
/// consumer(inout1 := refIB);  // error: IB EXTENDS IA, but the callee writes an IA
/// consumer(inout1 := refIC);  // error: IC unrelated to IA
/// ```
pub fn validate_byref_interface_argument(
    parameter_name: &str,
    expected_iface: &str,
    actual_iface: &str,
    location: &SourceLocation,
) -> Option<Diagnostic> {
    if expected_iface.eq_ignore_ascii_case(actual_iface) {
        return None;
    }

    Some(
        Diagnostic::new(format!(
            "Parameter {parameter_name} is passed by reference, it requires an argument of type {expected_iface} but got {actual_iface}"
        ))
        .with_error_code("E037")
        .with_location(location.clone()),
    )
}

/// Validates that an argument passed to a by-reference interface parameter is a variable, as a
/// by-reference parameter of any other type requires; the callee may keep the address, which a call
/// result's temporary does not survive. Returns `Some(Diagnostic)` if not.
///
/// ```st
/// consumer(inout1 := instance);       // ok:    a variable
/// consumer(inout1 := producer());     // error: a call result has no storage the callee may keep
/// consumer(inout1 := producer()[0]);  // error: an element of a call result lives in its temporary
/// consumer(inout1 := getPtr()^);      // ok:    the pointee of a returned pointer is not a temporary
/// ```
pub fn validate_byref_argument_is_variable(
    index: &Index,
    parameter: &VariableIndexEntry,
    argument: &AstNode,
) -> Option<Diagnostic> {
    if argument.can_be_assigned_to() && is_variable_backed(argument) {
        return None;
    }

    let parameter_kind =
        if index.find_effective_type_info(parameter.get_type_name()).is_some_and(|it| it.is_reference_to()) {
            "REFERENCE TO".to_string()
        } else {
            parameter.get_variable_type().to_string()
        };

    Some(
        Diagnostic::new(format!(
            "Expected a reference for parameter {} because their type is {parameter_kind}",
            parameter.get_name()
        ))
        .with_error_code("E031")
        .with_location(located(argument)),
    )
}

/// The location of `node`, or of its innermost located base when the node itself, such as an index
/// access on a call result, carries none.
fn located(node: &AstNode) -> SourceLocation {
    match &node.stmt {
        AstStatement::ReferenceExpr(ReferenceExpr { base: Some(base), .. })
            if node.location.is_undefined() =>
        {
            located(base)
        }
        _ => node.location.clone(),
    }
}

/// Whether `node` is a reference whose root is a variable, the instance itself or its parent rather
/// than a call result, so that its storage outlives the statement. A dereference names the storage
/// the pointer refers to, whatever produced the pointer, and a cast is judged by its operand, which
/// the parser keeps in parentheses.
fn is_variable_backed(node: &AstNode) -> bool {
    match &node.stmt {
        AstStatement::Identifier(_) | AstStatement::This | AstStatement::Super(_) => true,
        AstStatement::ParenExpression(inner) => is_variable_backed(inner),
        AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Deref, .. }) => true,
        AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Cast(operand), .. }) => {
            is_variable_backed(operand)
        }
        AstStatement::ReferenceExpr(ReferenceExpr { base, .. }) => {
            base.as_ref().is_none_or(|base| is_variable_backed(base))
        }
        _ => false,
    }
}

/// Validates that nothing is passed to a sized variadic whose element type is `REFERENCE TO` the
/// interface. The variadic stores each element by value in a pointer-sized slot, which neither an
/// interface reference nor the fat pointer wrapping an instance fits. Returns `Some(Diagnostic)` if it is.
///
/// ```st
/// consume(reference);  // error: the fat pointer does not fit the element
/// consume(instance);   // error: neither does the one wrapping the instance
/// ```
pub fn validate_variadic_takes_argument(
    index: &Index,
    parameter: &VariableIndexEntry,
    argument_type: &str,
    interface_name: &str,
    location: &SourceLocation,
) -> Option<Diagnostic> {
    let reference_element = parameter.is_variadic()
        && index.find_effective_type_info(parameter.get_type_name()).is_some_and(|it| it.is_reference_to());
    if !reference_element {
        return None;
    }

    Some(
        Diagnostic::new(format!(
            "Parameter {} is a sized variadic of REFERENCE TO {interface_name} and cannot take an argument of type {argument_type}",
            parameter.get_name()
        ))
        .with_error_code("E159")
        .with_location(location.clone()),
    )
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
