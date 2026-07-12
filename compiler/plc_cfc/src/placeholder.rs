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
//! Once the index and annotations exist, [`resolve_temp_types`] runs at `post_annotate`, decodes each
//! placeholder and rewrites it with the real type. Output pins are looked up in the index; return types
//! also come from the index for concrete callees, but a *generic* function (e.g. the builtin
//! `FUNCTION SEL<U: ANY> : U`) declares its return as a marker type (`__SEL__U`) that only a call site
//! can specialize — those are resolved from the annotated type of the call assigned to the temporary,
//! which is why this pass runs after annotation. Placeholders it cannot resolve are left untouched.

use std::collections::HashMap;

use ast::ast::{Assignment, AstStatement, CompilationUnit, DataTypeDeclaration};
use plc::index::Index;
use plc::resolver::AnnotationMap;

const RETURN_PREFIX: &str = "__return@";

const OUTPUT_PREFIX: &str = "__output@";

pub(crate) fn return_placeholder(type_name: &str) -> String {
    format!("{RETURN_PREFIX}{type_name}")
}

pub(crate) fn output_placeholder(type_name: &str, pin: &str) -> String {
    format!("{OUTPUT_PREFIX}{type_name}@{pin}")
}

/// Returns true if the unit declares any placeholder-typed temporary, i.e. was produced by the CFC
/// frontend and still needs [`resolve_temp_types`]. Cheap; lets the driver skip non-CFC projects.
pub fn has_placeholder_types(unit: &CompilationUnit) -> bool {
    unit.pous
        .iter()
        .flat_map(|pou| &pou.variable_blocks)
        .filter(|block| block.kind.is_local())
        .flat_map(|block| &block.variables)
        .filter_map(|variable| variable.data_type_declaration.get_referenced_type())
        .any(|referenced_type| decode(&referenced_type).is_some())
}

pub fn resolve_temp_types(
    unit: &mut CompilationUnit,
    index: &Index,
    annotations: &impl AnnotationMap,
) -> bool {
    let call_types = collect_call_types(unit, index, annotations);
    let mut changed = false;

    for pou in &mut unit.pous {
        for block in pou.variable_blocks.iter_mut().filter(|block| block.kind.is_local()) {
            for variable in &mut block.variables {
                let DataTypeDeclaration::Reference { referenced_type, .. } =
                    &mut variable.data_type_declaration
                else {
                    continue;
                };

                if let Some(resolved) = resolve(&variable.name, referenced_type, index, &call_types) {
                    *referenced_type = resolved;
                    changed = true;
                }
            }
        }
    }

    changed
}

/// Maps each variable assigned from an expression to that expression's annotated type. For a
/// temporary fed by a call this is the call's (per-call-site specialized) return type — the only
/// authority for generic callees, whose *declared* return type is just a generic marker.
fn collect_call_types(
    unit: &CompilationUnit,
    index: &Index,
    annotations: &impl AnnotationMap,
) -> HashMap<String, String> {
    let mut call_types = HashMap::new();
    for implementation in &unit.implementations {
        for statement in &implementation.statements {
            let AstStatement::Assignment(Assignment { left, right }) = statement.get_stmt() else {
                continue;
            };
            let Some(name) = left.get_flat_reference_name() else { continue };
            let Some(data_type) = annotations.get_type(right, index) else { continue };

            call_types.insert(name.to_string(), data_type.get_name().to_string());
        }
    }

    call_types
}

enum CfcTempType {
    Return { type_name: String },
    Output { type_name: String, pin: String },
}

fn resolve(
    variable_name: &str,
    referenced_type: &str,
    index: &Index,
    call_types: &HashMap<String, String>,
) -> Option<String> {
    match decode(referenced_type)? {
        CfcTempType::Return { type_name } => {
            let pou = index.find_pou(&type_name)?;

            // A generic function's declared return type is a marker (`__SEL__U`) no variable can be
            // declared with; the concrete type only exists at the call site, where the annotator
            // already specialized it.
            if pou.is_generic() {
                return call_types.get(variable_name).cloned();
            }

            pou.get_return_type().map(str::to_string)
        }

        CfcTempType::Output { type_name, pin } => {
            let member_type = index.find_member(&type_name, &pin)?.get_type_name();

            // A function models its VAR_OUTPUT members as auto-deref pointers (`__auto_pointer_to_X`),
            // unlike function blocks and programs whose output members are plain values. The variable
            // holding the pin's value must be declared with the inner value type instead.
            match index.find_effective_type_info(member_type) {
                Some(info) if info.is_auto_deref() => info.get_inner_pointer_type_name().map(str::to_string),
                _ => Some(member_type.to_string()),
            }
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
    use super::{output_placeholder, return_placeholder};
    use ast::ast::{CompilationUnit, DataTypeDeclaration, LinkageType, Variable, VariableBlock};
    use ast::provider::IdProvider;
    use plc::index::{Index, indexer};
    use plc::resolver::AnnotationMapImpl;
    use plc_source::source_location::{SourceLocation, SourceLocationFactory};

    /// The annotation-less test harness: resolves with an empty annotation map, which suffices for
    /// everything but generic callees (those are covered end-to-end in `tests/lit/cfc/generic_call`)
    fn resolve_temp_types(unit: &mut CompilationUnit, index: &Index) -> bool {
        super::resolve_temp_types(unit, index, &AnnotationMapImpl::default())
    }

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
        unit.pous[0].variable_blocks.push(VariableBlock::local(vec![Variable {
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
    fn output_pin_of_function_is_typed_by_value() {
        // A function's VAR_OUTPUT member is indexed as an auto-deref pointer (`__auto_pointer_to_DINT`);
        // the resolved type must be the inner value type, not the pointer wrapper
        let mut unit = unit_with_temp(output_placeholder("myFunc", "out"));
        let index = index_of(&[
            "FUNCTION myFunc : DINT VAR_INPUT a : DINT; END_VAR VAR_OUTPUT out : DINT; END_VAR myFunc := a; END_FUNCTION",
        ]);

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
