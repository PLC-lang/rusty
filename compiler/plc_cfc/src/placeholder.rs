//! Typing of the synthetic `__temp_N` variables the transpiler introduces for block outputs.
//!
//! A temp's type is the return type of a called function or the type of a called POU's output pin —
//! both live in *another* POU's signature, unknown while a `.cfc` is parsed on its own. The
//! transpiler therefore stamps each temp with a *placeholder* type encoding how to derive the real
//! type, and [`resolve_temp_types`] rewrites those placeholders once the global index exists. The
//! driver runs it as a post-index pipeline participant, after which the temps are ordinary typed
//! `VAR_TEMP` members. Both the format and its resolution live here, so the encoding never leaks.

use ast::ast::{CompilationUnit, DataTypeDeclaration};
use plc::index::Index;

/// Placeholder for "the return type of function `<type>`".
const RETURN_PREFIX: &str = "__return@";

/// Placeholder for "the type of output pin `<pin>` of POU `<type>`".
const OUTPUT_PREFIX: &str = "__output@";

pub(crate) fn return_placeholder(type_name: &str) -> String {
    format!("{RETURN_PREFIX}{type_name}")
}

pub(crate) fn output_placeholder(type_name: &str, pin: &str) -> String {
    format!("{OUTPUT_PREFIX}{type_name}@{pin}")
}

/// Rewrites every CFC temp's placeholder type to the concrete type named by `index`, returning
/// whether anything changed (so the caller can skip a re-index when no CFC temps were present). A
/// placeholder that fails to resolve is left untouched, surfacing later as an unresolved-type error.
pub fn resolve_temp_types(unit: &mut CompilationUnit, index: &Index) -> bool {
    let mut changed = false;

    for pou in &mut unit.pous {
        for block in pou.variable_blocks.iter_mut().filter(|block| block.kind.is_temp()) {
            for variable in &mut block.variables {
                let DataTypeDeclaration::Reference { referenced_type, .. } =
                    &mut variable.data_type_declaration
                else {
                    continue;
                };

                if let Some(resolved) = resolve(referenced_type, index) {
                    *referenced_type = resolved;
                    changed = true;
                }
            }
        }
    }

    changed
}

/// How a CFC temp's real type is derived, decoded from its placeholder.
enum CfcTempType {
    /// The return type of function `type_name`.
    Return { type_name: String },
    /// The type of output pin `pin` of POU `type_name`.
    Output { type_name: String, pin: String },
}

/// Resolves a temp's placeholder to a concrete type name via the index, or `None` if it is not a
/// placeholder or the referenced signature is absent.
fn resolve(referenced_type: &str, index: &Index) -> Option<String> {
    match decode(referenced_type)? {
        CfcTempType::Return { type_name } => {
            index.find_pou(&type_name)?.get_return_type().map(str::to_string)
        }

        CfcTempType::Output { type_name, pin } => {
            index.find_member(&type_name, &pin).map(|member| member.get_type_name().to_string())
        }
    }
}

/// Decodes a temp's placeholder type, or `None` if it is not a CFC temp placeholder.
fn decode(referenced_type: &str) -> Option<CfcTempType> {
    if let Some(type_name) = referenced_type.strip_prefix(RETURN_PREFIX) {
        return Some(CfcTempType::Return { type_name: type_name.to_string() });
    }

    if let Some(rest) = referenced_type.strip_prefix(OUTPUT_PREFIX) {
        let (type_name, pin) = rest.split_once('@')?;
        return Some(CfcTempType::Output { type_name: type_name.to_string(), pin: pin.to_string() });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{output_placeholder, resolve_temp_types, return_placeholder};
    use ast::ast::{CompilationUnit, DataTypeDeclaration, LinkageType, Variable, VariableBlock};
    use ast::provider::IdProvider;
    use plc::index::{Index, indexer};
    use plc_source::source_location::{SourceLocation, SourceLocationFactory};

    /// Parses an ST source into a compilation unit (diagnostics ignored).
    fn parse_st(src: &str) -> CompilationUnit {
        plc::parser::parse(
            plc::lexer::lex_with_ids(src, IdProvider::default(), SourceLocationFactory::internal(src)),
            LinkageType::Internal,
            "test",
        )
        .0
    }

    /// An index built from the given ST callee POUs.
    fn index_of(callees: &[&str]) -> Index {
        let mut index = Index::default();
        for callee in callees {
            index.import(indexer::index(&parse_st(callee)));
        }
        index
    }

    /// A scaffold POU hosting a single `VAR_TEMP __temp_0` of the given placeholder type — the shape the
    /// transpiler emits. The placeholder can't be written in ST text (the `@` isn't a valid type name),
    /// so it is injected directly.
    fn unit_with_temp(placeholder: String) -> CompilationUnit {
        let mut unit = parse_st("PROGRAM main\nEND_PROGRAM");
        unit.pous[0].variable_blocks.push(VariableBlock::temp(vec![Variable {
            name: "__temp_0".into(),
            data_type_declaration: DataTypeDeclaration::Reference {
                referenced_type: placeholder,
                location: SourceLocation::undefined(),
            },
            initializer: None,
            address: None,
            location: SourceLocation::undefined(),
        }]));
        unit
    }

    /// The resolved type name of the injected `__temp_0`.
    fn temp_type(unit: &CompilationUnit) -> Option<&str> {
        unit.pous[0].variable_blocks.last()?.variables[0].data_type_declaration.get_referenced_type()
    }

    #[test]
    fn function_result_temp_is_typed() {
        // `__return@myAdd` resolves to myAdd's DINT return type.
        let mut unit = unit_with_temp(return_placeholder("myAdd"));
        let index =
            index_of(&["FUNCTION myAdd : DINT VAR_INPUT a, b : DINT; END_VAR myAdd := a + b; END_FUNCTION"]);

        assert!(resolve_temp_types(&mut unit, &index));
        assert_eq!(temp_type(&unit), Some("DINT"));
    }

    #[test]
    fn output_pin_temp_is_typed() {
        // `__output@Counter@count` resolves to the type of Counter's `count` output.
        let mut unit = unit_with_temp(output_placeholder("Counter", "count"));
        let index =
            index_of(&["FUNCTION_BLOCK Counter VAR_OUTPUT count : DINT; END_VAR END_FUNCTION_BLOCK"]);

        assert!(resolve_temp_types(&mut unit, &index));
        assert_eq!(temp_type(&unit), Some("DINT"));
    }

    #[test]
    fn unresolvable_placeholder_is_left_untouched() {
        // A placeholder whose signature is absent from the index is left as-is, nothing reported changed.
        let placeholder = return_placeholder("missing");
        let mut unit = unit_with_temp(placeholder.clone());

        assert!(!resolve_temp_types(&mut unit, &index_of(&[])));
        assert_eq!(temp_type(&unit), Some(placeholder.as_str()));
    }
}
