//! Type resolution for generic-typed CFC temporaries.
//!
//! CFC ensures that no block is called more than once within a cycle. However, sometimes the output variable
//! or the return value of a block may need to be consumed by multiple consumers. For example think of
//! something like
//! ```text
//!         myAdd (0)
//!       +--------------------+
//! x --> | a            myAdd | --+--> result1 (1)
//! y --> | b                  |   '--> result2 (2)
//!       +--------------------+
//! ```
//!
//! There are two ways to fix this: (1) When dealing with stateful POUs directly access the member fields or
//! (2) when dealing with stateless POUs (functions, methods) create a "temporary" variable, persist the
//! return/output value in there and pass it to the consumers.
//!
//! This case distinction is necessary because a stateful POU persists its outputs, they live in the
//! instance and can be read back as `instance.member` at any time. A stateless POU has no instance thus its
//! outputs vanish the moment the call returns, and calling it once per consumer instead would duplicate
//! side effects (and possibly yield different results). So the caller has to persist the value itself.
//!
//! However, the stateless approach brings its own challenges. Let's inspect a simple example first to get
//! an understanding of the "temporary" variable creation:
//! ```text
//!         myAdd (0)
//!       +--------------------+
//! x --> | a            myAdd | --> result (1)
//! y --> | b          doubled | --> d (2)
//!       +--------------------+
//! ```
//! which transpiles into
//! ```iecst
//! VAR
//!     __out_myAdd_1   : DINT;
//!     __out_doubled_1 : DINT;
//! END_VAR
//!     __out_myAdd_1 := myAdd(a := x, b := y, doubled => __out_doubled_1);
//!     result := __out_myAdd_1;
//!     d := __out_doubled_1;
//! ```
//! The temporaries simply take the callee's declared types, `DINT` here. Now let's take that same example,
//! but make it generic, i.e. `myAdd<T: ANY_NUM> : T` where the inputs, the output variable and the return
//! value all bind `T`. This transpiles literally into
//! ```iecst
//! VAR
//!     __out_myAdd_1   : __myAdd__T;
//!     __out_doubled_1 : __myAdd__T;
//! END_VAR
//!     __out_myAdd_1 := myAdd(a := x, b := y, doubled => __out_doubled_1);
//!     result := __out_myAdd_1;
//!     d := __out_doubled_1;
//! ```
//!
//! The issue being, at transpile time we do not have enough type information to query the concrete type the
//! given "temporary" variable will receive. Instead, we have to resolve them ourselves which this module is
//! responsible for.
//!
//! The workflow can be described as collecting all generic variables that have not been resolved yet,
//! annotate the unit once, visit the assignment sites and try to derive the generic variables type if
//! possible based on the context. For example, given a chain of two `myAdd` calls, if we visit
//! `__out_myAdd_1 := ...` in
//! ```iecst
//! __out_myAdd_1 := myAdd(a := x, b := y, doubled => );                // x, y : INT
//! __out_myAdd_5 := myAdd(a := __out_myAdd_1, b := z, doubled => );    // z : DINT
//! ```
//! then the type is derivable by simply asking the annotator for the type of the call, `x` and `y` decide
//! `T = INT`. At the same time `__out_myAdd_5 := ...` is not derivable yet because it references a generic
//! variable that has not been resolved yet: deriving now would consider `z` alone and could disagree with
//! the final answer, so it has to wait one round until `__out_myAdd_1`'s resolved type is published to the
//! index and a fresh annotation can pick it up.
//!
//! This workflow is repeated until all generic variables have been resolved or we've hit a point where the
//! remaining variables are genuinely not resolvable. Not resolvable being something like a generic output
//! fed back into nothing but its own generic inputs, e.g.
//! ```text
//!          myAdd (0)
//!        +--------------------+
//!   .--> | a            myAdd | --+--> acc (1)
//!   +--> | b                  |   |
//!   |    +--------------------+   |
//!   '-----------------------------'
//! ```
//!
//! The downside of doing this with a participant rather than a dedicated IR is the round-based looping. If
//! we assume a unit has a very long chain of generic blocks like the following example then we have to
//! assume a total of (worst case) N rounds. One per link, because each link only becomes derivable once its
//! predecessor's type is published. Resolved types are published in place (`Index::update_member_type`), so
//! a round only costs one annotation of this unit rather than a re-index of the whole project.
//! ```text
//!       +------------+     +------------+     +------------+
//! x --> | a      out | --> | a      out | --> | a      out | --> ...
//! y --> | b          | z ->| b          | w ->| b          |
//!       +------------+     +------------+     +------------+
//! ```

use std::collections::{HashMap, HashSet};

use plc::index::Index;
use plc::resolver::{AnnotationMap, TypeAnnotator};
use plc::typesystem::{DataType, DataTypeInformation};
use plc_ast::ast::{
    flatten_expression_list, AstNode, AstStatement, CallStatement, CompilationUnit, DataTypeDeclaration,
    UnaryExpression, Variable,
};
use plc_ast::provider::IdProvider;
use plc_diagnostics::diagnostics::Diagnostic;

/// Runs one round; returns whether any temporary was patched, in both the
/// unit's declarations and the index, meaning another round could make
/// progress.
pub fn infer_temporary_types(unit: &mut CompilationUnit, index: &mut Index, ids: IdProvider) -> bool {
    // Temporaries whose declared type is still generic, the unknowns.
    let open = open_temporaries(unit, index).map(|variable| variable.name.clone()).collect::<HashSet<_>>();
    if open.is_empty() {
        return false;
    }
    log::trace!("round starts with {} open temporaries: {:?}", open.len(), {
        let mut names = open.iter().collect::<Vec<_>>();
        names.sort();
        names
    });

    // Annotate this unit only; declarations patched in earlier rounds are
    // already published to the index, so the answers improve round over round.
    let (annotations, ..) = TypeAnnotator::visit_unit(index, unit, ids);

    // Harvest the concrete type each open temporary's capture resolved to.
    let mut resolved = HashMap::new();
    for statement in unit.implementations.iter().flat_map(|implementation| &implementation.statements) {
        // `callee(...)`, bare or behind `temporary := callee(...)`.
        let (call_node, return_capture) = match statement.get_stmt() {
            AstStatement::Assignment(assignment) => {
                (assignment.right.as_ref(), assignment.left.get_flat_reference_name())
            }
            AstStatement::CallStatement(_) => (statement, None),
            _ => continue,
        };
        let AstStatement::CallStatement(call) = call_node.get_stmt() else { continue };

        // `temporary := callee(...)`: the temporary takes the call's annotated (return) type.
        if let Some(name) = return_capture {
            if open.contains(name) && is_ready(call, name, &open) {
                match concrete_type(annotations.get_type(call_node, index), index) {
                    Some(data_type) => {
                        log::trace!("`{name}` resolved to `{data_type}` (return capture)");
                        resolved.insert(name.to_string(), data_type);
                    }
                    None => log::trace!("`{name}`: call has no concrete annotation"),
                }
            }
        }

        // `parameter => temporary`: the temporary takes the output parameter's annotated type.
        for argument in arguments(call) {
            let AstStatement::OutputAssignment(inner) = argument.get_stmt() else { continue };
            let Some(name) = base_reference(&inner.right) else { continue };
            if open.contains(name) && is_ready(call, name, &open) {
                match concrete_type(annotations.get_type(&inner.left, index), index) {
                    Some(data_type) => {
                        log::trace!("`{name}` resolved to `{data_type}` (output capture)");
                        resolved.insert(name.to_string(), data_type);
                    }
                    None => log::trace!("`{name}`: output parameter has no concrete annotation"),
                }
            }
        }
    }

    // Patch the resolved declarations in place and publish them to the index,
    // making them visible to the next round's annotation without a re-index.
    for pou in &mut unit.pous {
        for variable in pou.variable_blocks.iter_mut().flat_map(|block| block.variables.iter_mut()) {
            if let Some(data_type) = resolved.get(&variable.name) {
                variable.data_type_declaration =
                    DataTypeDeclaration::reference(data_type, variable.location.clone());
                index.update_member_type(&pou.name, &variable.name, data_type);
            }
        }
    }

    log::trace!("round resolved {} of {} temporaries", resolved.len(), open.len());
    !resolved.is_empty()
}

/// One diagnostic per temporary the fixed point could not resolve, reported
/// in terms of the block the user placed, not the generated temporary.
pub fn unresolved_temporaries(unit: &CompilationUnit, index: &Index) -> Vec<Diagnostic> {
    open_temporaries(unit, index)
        .map(|variable| {
            let block = producer(unit, &variable.name).unwrap_or(variable.name.as_str());
            log::trace!("`{}` never resolved; reporting `{block}` as unresolvable", variable.name);
            Diagnostic::unresolved_generic_output(block, variable.location.clone())
        })
        .collect()
}

// Declarations whose type is still generic, the unknowns.
fn open_temporaries<'unit>(
    unit: &'unit CompilationUnit,
    index: &'unit Index,
) -> impl Iterator<Item = &'unit Variable> {
    unit.pous.iter().flat_map(|pou| &pou.variable_blocks).flat_map(|block| &block.variables).filter(
        |variable| {
            variable.data_type_declaration.get_name().is_some_and(|name| {
                matches!(index.get_type_information_or_void(name), DataTypeInformation::Generic { .. })
            })
        },
    )
}

/// May we trust the annotator about this call this round? Not yet if it reads
/// a *foreign* open temporary (partial evidence, wait for that producer), and
/// never if nothing external feeds it (left open, reported as unresolvable).
/// Reading its *own* output is fine; the external inputs decide the type.
fn is_ready(call: &CallStatement, own: &str, open: &HashSet<String>) -> bool {
    let mut external = false;
    for argument in arguments(call) {
        // Only input values feed candidates; output captures receive.
        let value = match argument.get_stmt() {
            AstStatement::Assignment(inner) => &inner.right,
            AstStatement::OutputAssignment(_) => continue,
            _ => argument,
        };

        match base_reference(value) {
            Some(temporary) if open.contains(temporary) => {
                // Another producer still open; defer.
                if temporary != own {
                    log::trace!("`{own}`: deferred, waits for `{temporary}`");
                    return false;
                }
            }
            _ => external |= !value.is_empty_statement(),
        }
    }

    if !external {
        log::trace!("`{own}`: no external input, cannot resolve");
    }
    external
}

// The annotator's answer, if usable: generic/void results are still undecided,
// and a type absent from the global index would dangle after the re-index.
fn concrete_type(data_type: Option<&DataType>, index: &Index) -> Option<String> {
    let data_type = data_type?;

    let generic = matches!(data_type.get_type_information(), DataTypeInformation::Generic { .. });
    (!generic && !data_type.is_void() && index.find_effective_type_by_name(data_type.get_name()).is_some())
        .then(|| data_type.get_name().to_string())
}

// The callee whose return or output is captured into the given temporary.
fn producer<'unit>(unit: &'unit CompilationUnit, temporary: &str) -> Option<&'unit str> {
    unit.implementations.iter().flat_map(|implementation| &implementation.statements).find_map(|statement| {
        // `callee(...)`, bare or behind `temporary := callee(...)`.
        let (call_node, return_capture) = match statement.get_stmt() {
            AstStatement::Assignment(assignment) => {
                (assignment.right.as_ref(), assignment.left.get_flat_reference_name())
            }
            _ => (statement, None),
        };
        let AstStatement::CallStatement(call) = call_node.get_stmt() else { return None };

        let captures = return_capture == Some(temporary)
            || arguments(call).iter().any(|argument| match argument.get_stmt() {
                AstStatement::OutputAssignment(inner) => base_reference(&inner.right) == Some(temporary),
                _ => false,
            });

        captures.then(|| call.operator.get_flat_reference_name()).flatten()
    })
}

// `NOT (x)` -> `x`: the referenced name behind negation/parentheses.
fn base_reference(node: &AstNode) -> Option<&str> {
    match node.get_stmt() {
        AstStatement::ParenExpression(inner) => base_reference(inner),
        AstStatement::UnaryExpression(UnaryExpression { value, .. }) => base_reference(value),
        _ => node.get_flat_reference_name(),
    }
}

// Flattened argument list of a call (empty for bare calls).
fn arguments(call: &CallStatement) -> Vec<&AstNode> {
    call.parameters.as_deref().map(flatten_expression_list).unwrap_or_default()
}
