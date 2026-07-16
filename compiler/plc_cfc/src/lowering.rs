//! Function-output temporaries and the resolution of their types.
//!
//! A stateless function's outputs don't persist, so `transpile` captures each
//! consumed output into a persistent temporary typed with a placeholder that
//! names the callee (`return@<fn>` / `output@<fn>@<pin>`). The concrete type is
//! unknown until the whole project is indexed; [`resolve`] then rewrites each
//! placeholder to the callee's real return/output type.

use std::collections::HashSet;

use plc::index::Index;
use plc_ast::ast::{CompilationUnit, DataTypeDeclaration, Variable};
use plc_source::source_location::SourceLocationFactory;

use crate::model::{FbdObject, Pin};
use crate::resolve::{Resolved, Statement};

/// A parsed temporary placeholder: which callee, and which of its outputs.
enum Placeholder<'a> {
    Return { function: &'a str },
    Output { function: &'a str, pin: &'a str },
}

/// One persistent temporary per consumed function output, ordered by the call
/// then its output pins — matching the order the outputs are captured.
pub(crate) fn temporaries(
    resolved: &Resolved,
    consumed: &HashSet<usize>,
    factory: &SourceLocationFactory,
) -> Vec<Variable> {
    let mut temporaries = Vec::new();
    for statement in &resolved.statements {
        let Statement::Call { block, .. } = statement else { continue };
        if !block.is_function() {
            continue;
        }
        for pin in block.output_pins().iter().filter(|pin| is_consumed(pin, consumed)) {
            temporaries.push(temp_variable(block, pin, factory));
        }
    }
    temporaries
}

/// Rewrite every function-output temporary's placeholder type to the callee's
/// real return/output type, now that the project is indexed. Returns whether any
/// placeholder was rewritten, so the caller can skip a re-index when none was.
pub fn resolve(units: &mut [CompilationUnit], index: &Index) -> bool {
    let mut rewritten = false;
    for unit in units {
        let variables = unit
            .pous
            .iter_mut()
            .flat_map(|pou| &mut pou.variable_blocks)
            .flat_map(|block| &mut block.variables);
        for variable in variables {
            let DataTypeDeclaration::Reference { referenced_type, .. } = &mut variable.data_type_declaration
            else {
                continue;
            };

            // A placeholder no callee/output resolves is left untouched;
            // annotation then reports the unknown type at the temporary itself.
            // TODO: emit a dedicated CFC diagnostic rather than leaning on that.
            if let Some(resolved) = parse(referenced_type).and_then(|placeholder| placeholder.lookup(index)) {
                *referenced_type = resolved;
                rewritten = true;
            }
        }
    }
    rewritten
}

pub(crate) fn temp_name(block: &FbdObject, pin: &Pin) -> String {
    format!("__out_{}_{}", pin.parameter_name, block.global_id)
}

pub(crate) fn is_consumed(pin: &Pin, consumed: &HashSet<usize>) -> bool {
    pin.output_pin().is_some_and(|id| consumed.contains(&id))
}

fn temp_variable(block: &FbdObject, pin: &Pin, factory: &SourceLocationFactory) -> Variable {
    let location = factory.create_block_location(block.global_id);
    Variable {
        name: temp_name(block, pin),
        data_type_declaration: DataTypeDeclaration::Reference {
            referenced_type: placeholder(block, pin),
            location: location.clone(),
        },
        initializer: None,
        address: None,
        location,
    }
}

/// The placeholder type a temporary carries until [`resolve`] rewrites it; it
/// names the callee so the concrete type can be looked up post-index.
// TODO: A type-inference marker (e.g. `__AUTO`) would be more elegant than
//       threading the callee's name through a textual placeholder.
fn placeholder(block: &FbdObject, pin: &Pin) -> String {
    let function = block.type_name().unwrap_or_default();
    match block.is_return_pin(pin) {
        true => format!("return@{function}"),
        false => format!("output@{function}@{}", pin.parameter_name),
    }
}

/// Parse a temporary's placeholder type; `None` for any ordinary type name.
fn parse(referenced_type: &str) -> Option<Placeholder<'_>> {
    if let Some(function) = referenced_type.strip_prefix("return@") {
        return Some(Placeholder::Return { function });
    }
    let (function, pin) = referenced_type.strip_prefix("output@")?.split_once('@')?;
    Some(Placeholder::Output { function, pin })
}

impl Placeholder<'_> {
    /// The callee's declared type name for this output, looked up in the index.
    /// A function's output is passed by reference, so its member type is an
    /// auto-deref pointer; the temporary needs the pointed-to (inner) type.
    fn lookup(&self, index: &Index) -> Option<String> {
        let variable = match self {
            Placeholder::Return { function } => index.find_return_variable(function),
            Placeholder::Output { function, pin } => index.find_member(function, pin),
        };
        let type_name = variable?.get_type_name();
        Some(index.get_type_information_or_void(type_name).get_inner_name().to_string())
    }
}
