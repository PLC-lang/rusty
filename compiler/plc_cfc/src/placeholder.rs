//! Type resolution of temporary variables
//!
//! The transpiler introduces temporary variables for results that are evaluated once but may be consumed many
//! times (e.g. a function's result feeding several sinks). Declaring such a temporary requires a type, but CFC
//! transpilation runs before the main compiler pipeline has built its index, so there is no way yet to tell
//! what type a function's return value or a block's output has, which is exactly the type the temporary needs.
//!
//! To get around this, the transpiler declares each temporary with a placeholder type name that encodes where
//! its real type will be found:
//!
//! - `__return@<pou>` is the return type of function `<pou>`.
//! - `__output@<pou>@<pin>` is the type of the output member `<pin>` on `<pou>`.
//!
//! Once the index exists, [`resolve_temp_types`] runs at `post_index`, decodes each placeholder and rewrites it
//! with the real type looked up from the index. Placeholders it cannot resolve are left untouched.

use ast::ast::{CompilationUnit, DataTypeDeclaration};
use plc::index::Index;

const RETURN_PREFIX: &str = "__return@";

const OUTPUT_PREFIX: &str = "__output@";

pub(crate) fn return_placeholder(type_name: &str) -> String {
    format!("{RETURN_PREFIX}{type_name}")
}

pub(crate) fn output_placeholder(type_name: &str, pin: &str) -> String {
    format!("{OUTPUT_PREFIX}{type_name}@{pin}")
}

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

enum CfcTempType {
    Return { type_name: String },
    Output { type_name: String, pin: String },
}

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

    fn parse_st(src: &str) -> CompilationUnit {
        plc::parser::parse(
            plc::lexer::lex_with_ids(src, IdProvider::default(), SourceLocationFactory::internal(src)),
            LinkageType::Internal,
            "test",
        )
        .0
    }

    fn index_of(callees: &[&str]) -> Index {
        let mut index = Index::default();
        for callee in callees {
            index.import(indexer::index(&parse_st(callee)));
        }
        index
    }

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

    fn temp_type(unit: &CompilationUnit) -> Option<&str> {
        unit.pous[0].variable_blocks.last()?.variables[0].data_type_declaration.get_referenced_type()
    }

    #[test]
    fn function_result_temp_is_typed() {
        let mut unit = unit_with_temp(return_placeholder("myAdd"));
        let index =
            index_of(&["FUNCTION myAdd : DINT VAR_INPUT a, b : DINT; END_VAR myAdd := a + b; END_FUNCTION"]);

        assert!(resolve_temp_types(&mut unit, &index));
        assert_eq!(temp_type(&unit), Some("DINT"));
    }

    #[test]
    fn output_pin_temp_is_typed() {
        let mut unit = unit_with_temp(output_placeholder("Counter", "count"));
        let index = index_of(&["FUNCTION_BLOCK Counter VAR_OUTPUT count : DINT; END_VAR END_FUNCTION_BLOCK"]);

        assert!(resolve_temp_types(&mut unit, &index));
        assert_eq!(temp_type(&unit), Some("DINT"));
    }

    #[test]
    fn unresolvable_placeholder_is_left_untouched() {
        let placeholder = return_placeholder("missing");
        let mut unit = unit_with_temp(placeholder.clone());

        assert!(!resolve_temp_types(&mut unit, &index_of(&[])));
        assert_eq!(temp_type(&unit), Some(placeholder.as_str()));
    }
}
