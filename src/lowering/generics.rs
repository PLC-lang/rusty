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
//! Builtin generics (`MUX`/`SEL`/`SHL`/…) are resolved inline by codegen and have no monomorphization,
//! so their resolution stays in the annotator; this phase handles only non-builtin (user/stdlib)
//! generic functions. It runs as a `post_annotate` pipeline participant before the aggregate-return
//! lowerer (see the `PipelineParticipantMut` impl in the driver crate).

use plc_ast::{
    ast::{
        flatten_expression_list, AstNode, AstStatement, CallStatement, CompilationUnit, DataType,
        DataTypeDeclaration, Implementation, LinkageType, Pou,
    },
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
    try_from_mut,
};
use plc_diagnostics::diagnostics::Diagnostic;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{
    builtins,
    index::{Index, PouIndexEntry},
    lowering::helper::{create_member_reference_with_location, new_implementation, new_pou},
    resolver::{
        generics::{
            derive_generic_types, generic_name_resolver, get_generic_candidate,
            get_specific_function_annotation,
        },
        AnnotationMap, StatementAnnotation,
    },
    typesystem::DataTypeInformation,
    validation::statement::evaluate_generic_nature_violation,
};

/// Resolves a type-name reference in a generic template to its concrete monomorph type.
///
/// In the indexed template, a generic parameter `T` on `foo` is referenced by its qualified index
/// name (`__foo__T`), not the bare symbol — so a direct `subst["T"]` lookup misses. This resolves the
/// reference through the index: if it is a `Generic { generic_symbol }`, the symbol is what `subst`
/// (built from the generic map, keyed by symbol) is keyed by. A bare-symbol key is also honoured as a
/// fallback. Returns `None` when the reference is not a generic to substitute.
fn resolve_concrete(index: &Index, subst: &FxHashMap<String, String>, type_name: &str) -> Option<String> {
    if let Some(concrete) = subst.get(type_name) {
        return Some(concrete.clone());
    }
    if let Some(DataTypeInformation::Generic { generic_symbol, .. }) =
        index.find_effective_type_info(type_name)
    {
        return subst.get(generic_symbol).cloned();
    }
    None
}

/// Substitutes generic-type references in a type declaration with their concrete type names.
/// `subst` maps a generic symbol (e.g. `"T"`) to a concrete type name (e.g. `"DINT"`); references are
/// resolved through the index (see [`resolve_concrete`]) and left untouched when not generic. Recurses
/// through array/pointer/vararg element types so shapes like `ARRAY[..] OF T`, `POINTER TO T` and
/// `{sized} T...` are substituted in place.
fn substitute_type_decl(
    index: &Index,
    decl: &DataTypeDeclaration,
    subst: &FxHashMap<String, String>,
) -> DataTypeDeclaration {
    match decl {
        DataTypeDeclaration::Reference { referenced_type, location } => DataTypeDeclaration::Reference {
            referenced_type: resolve_concrete(index, subst, referenced_type)
                .unwrap_or_else(|| referenced_type.clone()),
            location: location.clone(),
        },
        DataTypeDeclaration::Aggregate { referenced_type, location } => DataTypeDeclaration::Aggregate {
            referenced_type: resolve_concrete(index, subst, referenced_type)
                .unwrap_or_else(|| referenced_type.clone()),
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
                data_type: Box::new(substitute_data_type(index, data_type, subst)),
                location: location.clone(),
                scope: scope.clone(),
            }
        }
    }
}

fn substitute_data_type(index: &Index, data_type: &DataType, subst: &FxHashMap<String, String>) -> DataType {
    match data_type {
        DataType::ArrayType { name, bounds, referenced_type, is_variable_length } => DataType::ArrayType {
            name: name.clone(),
            bounds: bounds.clone(),
            referenced_type: Box::new(substitute_type_decl(index, referenced_type, subst)),
            is_variable_length: *is_variable_length,
        },
        DataType::PointerType { name, referenced_type, auto_deref, type_safe, is_function } => {
            DataType::PointerType {
                name: name.clone(),
                referenced_type: Box::new(substitute_type_decl(index, referenced_type, subst)),
                auto_deref: *auto_deref,
                type_safe: *type_safe,
                is_function: *is_function,
            }
        }
        DataType::VarArgs { referenced_type, sized } => DataType::VarArgs {
            referenced_type: referenced_type
                .as_ref()
                .map(|it| Box::new(substitute_type_decl(index, it, subst))),
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
pub(crate) fn materialize_monomorph(
    index: &Index,
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
                variable.data_type_declaration =
                    substitute_type_decl(index, &variable.data_type_declaration, subst);
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
    pou.return_type = template.return_type.as_ref().map(|rt| substitute_type_decl(index, rt, subst));

    let implementation = new_implementation(
        monomorph_name,
        vec![],
        pou.kind.clone(),
        LinkageType::External,
        template.location.clone(),
    );
    (pou, implementation)
}

/// Lowering phase that rewrites generic function calls to their concrete monomorphizations and
/// materializes any missing monomorph as an `{external}` POU declaration.
///
/// Runs as a `post_annotate` pipeline participant before the aggregate-return lowerer; the driver
/// drives the re-index/re-annotate fixed point (see the `PipelineParticipantMut` impl in the driver
/// crate). By the time this runs, generic resolution no longer happens in the annotator's first pass
/// — this phase is the single place generics are resolved and monomorphs are created.
#[derive(Default)]
pub struct GenericLowerer {
    pub index: Option<Index>,
    pub annotation: Option<Box<dyn AnnotationMap>>,
    pub id_provider: IdProvider,
    pub diagnostics: Vec<Diagnostic>,
    /// Monomorph names already resolved this run (materialized or found real) — dedup guard,
    /// accumulated across fixed-point passes.
    provided: FxHashSet<String>,
    /// `(template_name, monomorph_name, symbol->concrete)` collected during a walk, materialized
    /// afterwards (once the walk's immutable borrow of the units is released).
    pending: Vec<(String, String, FxHashMap<String, String>)>,
    /// Operator ids of calls already reported for an `E062` nature violation — dedup guard so a
    /// left-unresolved invalid call isn't re-reported on subsequent fixed-point passes.
    reported: FxHashSet<usize>,
    /// Whether the current pass rewrote a call or queued a monomorph (⇒ re-index and run again).
    changed: bool,
}

impl GenericLowerer {
    pub fn new(id_provider: IdProvider) -> Self {
        Self { id_provider, ..Default::default() }
    }

    /// Walks all units rewriting resolvable generic calls to concrete monomorphs and queueing any
    /// missing monomorph, then materializes the queued monomorphs. Returns `true` if anything
    /// changed, in which case the caller must re-index/re-annotate and call `lower` again.
    pub fn lower(&mut self, units: &mut [CompilationUnit]) -> bool {
        if self.index.is_none() || self.annotation.is_none() {
            return false;
        }
        self.changed = false;
        self.pending.clear();
        for unit in units.iter_mut() {
            self.visit_compilation_unit(unit);
        }
        self.materialize_pending(units);
        self.changed
    }

    fn materialize_pending(&mut self, units: &mut [CompilationUnit]) {
        for (template_name, monomorph_name, subst) in std::mem::take(&mut self.pending) {
            let materialized = {
                let Some(template) = units.iter().flat_map(|it| &it.pous).find(|it| it.name == template_name)
                else {
                    continue;
                };
                let index = self.index.as_ref().expect("index set during lowering");
                materialize_monomorph(index, template, &monomorph_name, &subst, &mut self.id_provider)
            };
            if let Some(unit) = units.iter_mut().find(|it| it.pous.iter().any(|p| p.name == template_name)) {
                unit.pous.push(materialized.0);
                unit.implementations.push(materialized.1);
            }
        }
    }

    fn lower_call(&mut self, statement: &mut CallStatement) {
        let resolved = {
            let index = self.index.as_ref().expect("index set during lowering");
            let annotations = self.annotation.as_deref().expect("annotations set during lowering");

            let Some(StatementAnnotation::Function { qualified_name, return_type, .. }) =
                annotations.get(&statement.operator)
            else {
                return;
            };
            let Some(PouIndexEntry::Function { generics, .. }) = index.find_pou(qualified_name) else {
                return;
            };
            // Non-generic call, or a builtin (MUX/SEL/MOVE/ADD/…) resolved specially by codegen:
            // leave it untouched.
            if generics.is_empty() || builtins::get_builtin(qualified_name).is_some() {
                return;
            }

            let qualified_name = qualified_name.clone();
            let return_type = return_type.clone();
            let generics = generics.clone();
            let args: Vec<&AstNode> =
                statement.parameters.as_deref().map(flatten_expression_list).unwrap_or_default();
            let pairs = paired_args(index, &qualified_name, &args);

            let candidates = pairs.iter().fold(
                FxHashMap::<String, Vec<String>>::default(),
                |mut acc, (param_type, arg)| {
                    if let Some((symbol, concrete)) =
                        get_generic_candidate(index, annotations, param_type, arg)
                    {
                        acc.entry(symbol.to_string()).or_default().push(concrete.to_string());
                    }
                    acc
                },
            );
            let generic_map = derive_generic_types(index, index, &generics, candidates);

            // E062: report each argument whose type does not satisfy its generic parameter's nature.
            // The hint is the *resolved* monomorph parameter type (from `generic_map`), so that an
            // e.g. INT argument to an `ANY_REAL` parameter — which resolves to REAL — is allowed,
            // while a REAL argument to an `ANY_INT` parameter is rejected. This mirrors the check the
            // validator performed against the first pass's resolved type hint pre-refactor.
            let mut diagnostics = Vec::new();
            for (param_type, arg) in &pairs {
                let Some((symbol, _)) = get_generic_candidate(index, annotations, param_type, arg) else {
                    continue;
                };
                let Some(binding) = generics.iter().find(|it| it.name == symbol) else { continue };
                let value = match arg.get_stmt() {
                    AstStatement::Assignment(data) | AstStatement::OutputAssignment(data) => &data.right,
                    _ => *arg,
                };
                let Some(actual) = annotations.get_type(value, index) else { continue };
                let hint = generic_map
                    .get(symbol)
                    .and_then(|ty| index.find_effective_type_by_name(ty.derived_type()))
                    .or_else(|| annotations.get_type_hint(value, index))
                    .unwrap_or(actual);
                if let Some(diagnostic) =
                    evaluate_generic_nature_violation(actual, hint, binding.nature, index, value)
                {
                    diagnostics.push(diagnostic);
                }
            }

            // Only lower when every generic parameter resolved to a concrete (non-generic) type;
            // otherwise this call isn't monomorphizable yet (e.g. an argument is itself generic) and
            // is left untouched.
            let fully_resolved = generics.iter().all(|binding| {
                generic_map.get(&binding.name).is_some_and(|ty| {
                    !matches!(
                        index.find_effective_type_info(ty.derived_type()),
                        Some(DataTypeInformation::Generic { .. })
                    )
                })
            });
            if !fully_resolved {
                return;
            }
            let (call_name, _) = get_specific_function_annotation(
                index,
                &generics,
                &qualified_name,
                &return_type,
                &generic_map,
                generic_name_resolver,
            );
            let already_real = index.find_pou(&call_name).filter(|it| !it.is_generic()).is_some();
            let subst = generic_map
                .iter()
                .map(|(symbol, ty)| (symbol.clone(), ty.derived_type().to_string()))
                .collect::<FxHashMap<_, _>>();
            (qualified_name, call_name, subst, already_real, diagnostics)
        };

        let (template_name, call_name, subst, already_real, diagnostics) = resolved;

        // A nature violation makes the call invalid: report E062 (once, even across fixed-point
        // passes) and leave the call unresolved. Materializing a nonsensical monomorph and rewriting
        // the operator would additionally coerce the offending argument and emit spurious downcast
        // warnings on top of the real error.
        if !diagnostics.is_empty() {
            if self.reported.insert(statement.operator.get_id()) {
                self.diagnostics.extend(diagnostics);
            }
            return;
        }

        // Queue materialization once per monomorph that has no real POU providing it.
        if self.provided.insert(call_name.clone()) && !already_real {
            self.pending.push((template_name, call_name.clone(), subst));
        }

        let location = statement.operator.get_location();
        *statement.operator =
            create_member_reference_with_location(&call_name, self.id_provider.clone(), None, location);
        self.changed = true;
    }
}

/// Pairs each call argument with the declared type name of the parameter it binds to, handling
/// positional, named and variadic (`...`) arguments — the same pairing the annotator uses when
/// collecting generic candidates.
fn paired_args<'a>(index: &Index, pou_name: &str, args: &[&'a AstNode]) -> Vec<(String, &'a AstNode)> {
    let params = index.get_available_parameters(pou_name);
    let mut pairs = Vec::new();
    let has_named = args.iter().any(|it| it.is_assignment() || it.is_output_assignment());
    if has_named {
        for arg in args {
            let Some(name) = arg.get_assignment_identifier() else { continue };
            if let Some(param) = params.iter().find(|it| it.get_name().eq_ignore_ascii_case(name)) {
                pairs.push((param.get_type_name().to_string(), *arg));
            }
        }
    } else {
        for (param, arg) in params.iter().zip(args.iter()) {
            pairs.push((param.get_type_name().to_string(), *arg));
        }
        if let Some(vararg) = index.get_variadic_member(pou_name) {
            for arg in args.iter().skip(params.len()) {
                pairs.push((vararg.get_type_name().to_string(), *arg));
            }
        }
    }
    pairs
}

impl AstVisitorMut for GenericLowerer {
    fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        if self.index.is_none() {
            return;
        }
        unit.walk(self);
    }

    fn visit_implementation(&mut self, implementation: &mut plc_ast::ast::Implementation) {
        // Skip the bodies of generic templates: their calls operate on still-generic types (`T`) and
        // are never code-generated directly (only their provided monomorphizations are). Lowering
        // them would mint bogus `FN__T` monomorphs.
        let is_generic_template = self
            .index
            .as_ref()
            .and_then(|index| index.find_pou(&implementation.name))
            .is_some_and(|pou| pou.is_generic());
        if is_generic_template {
            return;
        }
        implementation.walk(self);
    }

    fn visit_call_statement(&mut self, node: &mut AstNode) {
        let statement = try_from_mut!(node, CallStatement).expect("CallStatement");
        // Lower nested/argument calls first (post-order) so an inner monomorph is resolved before the
        // outer call's argument types are consulted.
        statement.walk(self);
        self.lower_call(statement);
    }
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
        let out = substitute_type_decl(&Index::default(), &reference("T"), &subst(&[("T", "DINT")]));
        assert!(
            matches!(out, DataTypeDeclaration::Reference { referenced_type, .. } if referenced_type == "DINT")
        );
    }

    #[test]
    fn leaves_non_generic_reference_untouched() {
        let out = substitute_type_decl(&Index::default(), &reference("DINT"), &subst(&[("T", "INT")]));
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
            substitute_type_decl(&Index::default(), &decl, &subst(&[("T", "STRING")]))
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

        let (pou, imp) = materialize_monomorph(
            &Index::default(),
            &template,
            "foo__DINT",
            &subst(&[("T", "DINT")]),
            &mut ids,
        );

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
