//! Lowers generic function calls to concrete monomorphizations.
//!
//! For each call to a generic function (e.g. `TO_STRING <T: ANY>` invoked with a `DINT`) this phase
//! resolves the concrete monomorphization name (`TO_STRING__DINT`), and — when no real POU already
//! provides it — **materializes** it as an `{external}` POU declaration in the AST, then rewrites
//! the call to target the concrete name.
//!
//! Because monomorphizations become real (external) AST POUs, they survive the subsequent
//! re-index/re-annotate, downstream passes (aggregate-return lowering, codegen) only ever see
//! concrete calls, and a genuinely-missing monomorphization becomes a link-time undefined symbol —
//! uniform for scalar and aggregate returns. This replaces the previous approach where generic
//! resolution ran inside the annotator's first pass and registered synthetic monomorphs into
//! `new_index` (which did not survive a re-index).
//!
//! This module currently provides the reusable core (type substitution + monomorph materialization);
//! it is wired into the pipeline as a participant in a following step.

use plc_ast::{
    ast::{DataType, DataTypeDeclaration, Implementation, LinkageType, Pou},
    provider::IdProvider,
};
use rustc_hash::FxHashMap;

use crate::lowering::helper::{new_implementation, new_pou};

// NOTE: these are exercised by unit tests and wired into the pipeline by the `GenericLowerer`
// participant in the following step; `allow(dead_code)` until then.

/// Substitutes generic-symbol references in a type declaration with their concrete type names.
/// `subst` maps a generic symbol (e.g. `"T"`) to a concrete type name (e.g. `"DINT"`); references
/// not in the map are left untouched. Recurses through array/pointer/vararg element types so shapes
/// like `ARRAY[..] OF T`, `POINTER TO T` and `{sized} T...` are substituted in place.
#[allow(dead_code)]
fn substitute_type_decl(
    decl: &DataTypeDeclaration,
    subst: &FxHashMap<String, String>,
) -> DataTypeDeclaration {
    match decl {
        DataTypeDeclaration::Reference { referenced_type, location } => DataTypeDeclaration::Reference {
            referenced_type: subst.get(referenced_type).cloned().unwrap_or_else(|| referenced_type.clone()),
            location: location.clone(),
        },
        DataTypeDeclaration::Aggregate { referenced_type, location } => DataTypeDeclaration::Aggregate {
            referenced_type: subst.get(referenced_type).cloned().unwrap_or_else(|| referenced_type.clone()),
            location: location.clone(),
        },
        DataTypeDeclaration::Definition { data_type, location, scope } => {
            // A bare generic placeholder collapses to a plain reference to its concrete type.
            if let DataType::GenericType { generic_symbol, .. } = data_type.as_ref() {
                if let Some(concrete) = subst.get(generic_symbol) {
                    return DataTypeDeclaration::Reference {
                        referenced_type: concrete.clone(),
                        location: location.clone(),
                    };
                }
            }
            DataTypeDeclaration::Definition {
                data_type: Box::new(substitute_data_type(data_type, subst)),
                location: location.clone(),
                scope: scope.clone(),
            }
        }
    }
}

#[allow(dead_code)]
fn substitute_data_type(data_type: &DataType, subst: &FxHashMap<String, String>) -> DataType {
    match data_type {
        DataType::ArrayType { name, bounds, referenced_type, is_variable_length } => DataType::ArrayType {
            name: name.clone(),
            bounds: bounds.clone(),
            referenced_type: Box::new(substitute_type_decl(referenced_type, subst)),
            is_variable_length: *is_variable_length,
        },
        DataType::PointerType { name, referenced_type, auto_deref, type_safe, is_function } => {
            DataType::PointerType {
                name: name.clone(),
                referenced_type: Box::new(substitute_type_decl(referenced_type, subst)),
                auto_deref: *auto_deref,
                type_safe: *type_safe,
                is_function: *is_function,
            }
        }
        DataType::VarArgs { referenced_type, sized } => DataType::VarArgs {
            referenced_type: referenced_type.as_ref().map(|it| Box::new(substitute_type_decl(it, subst))),
            sized: *sized,
        },
        // Struct/enum/subrange/string/generic-placeholder carry no substitutable element reference
        // in a generic function signature; clone as-is.
        other => other.clone(),
    }
}

/// Builds a concrete `{external}` monomorphization (`Pou` + empty-body `Implementation`) from a
/// generic template by substituting each generic symbol with its concrete type. The result is a
/// declaration only: codegen emits an external symbol that the linker resolves against the provided
/// implementation (e.g. the stdlib `.so`) or rejects if genuinely missing. Aggregate return types
/// are preserved so the aggregate-return lowerer transforms them uniformly afterwards.
#[allow(dead_code)]
pub(crate) fn materialize_monomorph(
    template: &Pou,
    monomorph_name: &str,
    subst: &FxHashMap<String, String>,
    id_provider: &mut IdProvider,
) -> (Pou, Implementation) {
    // `Pou` is not `Clone`, so rebuild it from the template's (clonable) blocks with substituted
    // element types, via `new_pou` (which fills the remaining fields with monomorph-appropriate
    // defaults: no generics, no poly/super/interfaces).
    let variable_blocks = template
        .variable_blocks
        .iter()
        .map(|block| {
            let mut block = block.clone();
            for variable in &mut block.variables {
                variable.data_type_declaration = substitute_type_decl(&variable.data_type_declaration, subst);
            }
            block
        })
        .collect();

    let mut pou = new_pou(
        monomorph_name,
        id_provider.next_id(),
        variable_blocks,
        template.kind.clone(),
        LinkageType::External,
        &template.location,
    );
    pou.return_type = template.return_type.as_ref().map(|rt| substitute_type_decl(rt, subst));

    let implementation = new_implementation(
        monomorph_name,
        vec![],
        pou.kind.clone(),
        LinkageType::External,
        template.location.clone(),
    );
    (pou, implementation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use plc_ast::{
        ast::{ArgumentProperty, PouType, VariableBlock, VariableBlockType},
        provider::IdProvider,
    };
    use plc_source::source_location::SourceLocation;
    use rustc_hash::FxHashMap;

    use crate::lowering::helper::{new_pou, new_variable};

    fn subst(pairs: &[(&str, &str)]) -> FxHashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    fn reference(name: &str) -> DataTypeDeclaration {
        DataTypeDeclaration::Reference { referenced_type: name.into(), location: SourceLocation::internal() }
    }

    #[test]
    fn substitutes_matching_reference() {
        let out = substitute_type_decl(&reference("T"), &subst(&[("T", "DINT")]));
        assert!(
            matches!(out, DataTypeDeclaration::Reference { referenced_type, .. } if referenced_type == "DINT")
        );
    }

    #[test]
    fn leaves_non_generic_reference_untouched() {
        let out = substitute_type_decl(&reference("DINT"), &subst(&[("T", "INT")]));
        assert!(
            matches!(out, DataTypeDeclaration::Reference { referenced_type, .. } if referenced_type == "DINT")
        );
    }

    #[test]
    fn substitutes_inside_sized_varargs() {
        // Mirrors `args : {sized} T...` (as in CONCAT/MAX): only the element type is substituted, the
        // `sized` flag is preserved.
        let decl = DataTypeDeclaration::Definition {
            data_type: Box::new(DataType::VarArgs {
                referenced_type: Some(Box::new(reference("T"))),
                sized: true,
            }),
            location: SourceLocation::internal(),
            scope: None,
        };
        let DataTypeDeclaration::Definition { data_type, .. } =
            substitute_type_decl(&decl, &subst(&[("T", "STRING")]))
        else {
            panic!("expected definition");
        };
        let DataType::VarArgs { referenced_type, sized } = *data_type else { panic!("expected varargs") };
        assert!(sized);
        assert!(
            matches!(referenced_type.as_deref(), Some(DataTypeDeclaration::Reference { referenced_type, .. }) if referenced_type == "STRING")
        );
    }

    #[test]
    fn materializes_external_monomorph_with_substituted_types() {
        let mut ids = IdProvider::default();
        // template: FUNCTION foo <T> : T VAR_INPUT x : T; END_VAR
        let mut template = new_pou(
            "foo",
            ids.next_id(),
            vec![VariableBlock::default()
                .with_block_type(VariableBlockType::Input(ArgumentProperty::ByVal))
                .with_variables(vec![new_variable("x", "T")])],
            PouType::Function,
            LinkageType::Internal,
            &SourceLocation::internal(),
        );
        template.return_type = Some(reference("T"));

        let (pou, imp) = materialize_monomorph(&template, "foo__DINT", &subst(&[("T", "DINT")]), &mut ids);

        assert_eq!(pou.name, "foo__DINT");
        assert_eq!(pou.linkage, LinkageType::External);
        assert!(pou.generics.is_empty());
        assert_eq!(pou.variable_blocks[0].variables[0].data_type_declaration.get_name(), Some("DINT"));
        assert_eq!(pou.return_type.as_ref().and_then(|it| it.get_name()), Some("DINT"));
        // declaration only
        assert_eq!(imp.name, "foo__DINT");
        assert_eq!(imp.linkage, LinkageType::External);
        assert!(imp.statements.is_empty());
    }
}
