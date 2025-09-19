use plc_ast::ast::{AstNode, CompilationUnit, DirectAccessType};
use plc_derive::Validators;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_index::GlobalContext;
use plc_source::source_location::SourceLocation;
use property::visit_property;
use rustc_hash::FxHashMap;
use variable::visit_config_variable;

use crate::{
    expression_path::ExpressionPath,
    index::{
        const_expressions::{ConstExpression, UnresolvableKind},
        FxIndexSet, Index, PouIndexEntry,
    },
    resolver::AnnotationMap,
    typesystem::DataType,
};

use self::{
    global::GlobalValidator,
    pou::{visit_implementation, visit_interface, visit_pou},
    recursive::RecursiveValidator,
    types::visit_user_type_declaration,
    variable::visit_variable_block,
};

mod array;
mod global;
mod pou;
mod property;
mod recursive;
pub(crate) mod statement;
mod types;
mod variable;

#[cfg(test)]
mod tests;

pub struct ValidationContext<'s, T: AnnotationMap> {
    annotations: &'s T,
    index: &'s Index,
    /// the type_name of the context for a reference (e.g. `a.b` where `a`'s type is the context of `b`)
    qualifier: Option<&'s str>,
    is_call: bool,
    is_cast: bool,
}

impl<'s, T: AnnotationMap> ValidationContext<'s, T> {
    fn with_qualifier(&self, qualifier: &'s str) -> Self {
        ValidationContext { qualifier: Some(qualifier), ..self.to_owned() }
    }

    fn with_optional_qualifier(&self, qualifier: Option<&'s str>) -> Self {
        ValidationContext { qualifier, ..self.to_owned() }
    }

    fn find_pou(&self, stmt: &AstNode) -> Option<&PouIndexEntry> {
        self.annotations.get_call_name(stmt).and_then(|pou_name| {
            self.index
                // check if this is an instance of a function block and get the type name
                .find_callable_instance_variable(self.qualifier, &[pou_name])
                .map(|it| it.get_type_name())
                // if it is not an instance, check if we are dealing with an action and get the base POU name
                .or_else(|| self.index.find_implementation_by_name(pou_name).map(|it| it.get_type_name()))
                // we didn't encounter an instance or action call, keep initial name
                .or(Some(pou_name))
                .and_then(|name| self.index.find_pou(name))
        })
    }

    fn set_is_call(&self) -> Self {
        ValidationContext { is_call: true, ..self.to_owned() }
    }

    fn is_call(&self) -> bool {
        self.is_call
    }

    fn set_cast(&self) -> Self {
        ValidationContext { is_cast: true, ..self.to_owned() }
    }
}

impl<T: AnnotationMap> Clone for ValidationContext<'_, T> {
    fn clone(&self) -> Self {
        ValidationContext {
            annotations: self.annotations,
            index: self.index,
            qualifier: self.qualifier,
            is_call: self.is_call,
            is_cast: self.is_cast,
        }
    }
}

/// This trait should be implemented by any validator used by `validation::Validator`
pub trait Validators {
    fn push_diagnostic(&mut self, diagnostic: Diagnostic);

    fn take_diagnostics(&mut self) -> Vec<Diagnostic>;
}

pub struct Validator<'a> {
    context: &'a GlobalContext,
    diagnostics: Vec<Diagnostic>,
    global_validator: GlobalValidator,
    recursive_validator: RecursiveValidator,
}

impl Validators for Validator<'_> {
    fn push_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }
    fn take_diagnostics(&mut self) -> Vec<Diagnostic> {
        std::mem::take(&mut self.diagnostics)
    }
}

impl<'a> Validator<'a> {
    pub fn new(context: &'a GlobalContext) -> Validator<'a> {
        Validator {
            context,
            diagnostics: Vec::new(),
            global_validator: GlobalValidator::new(),
            recursive_validator: RecursiveValidator::new(),
        }
    }

    pub fn get_type_name_or_slice(&self, dt: &DataType) -> String {
        if dt.is_internal() {
            return dt.get_type_information().get_inner_name().to_string();
        }

        self.context.slice(&dt.location)
    }

    pub fn diagnostics(&mut self) -> Vec<Diagnostic> {
        let mut all_diagnostics = Vec::new();
        all_diagnostics.append(&mut self.take_diagnostics());
        all_diagnostics.append(&mut self.global_validator.take_diagnostics());
        all_diagnostics.append(&mut self.recursive_validator.take_diagnostics());
        all_diagnostics
    }

    pub fn perform_global_validation(&mut self, index: &Index) {
        self.global_validator.validate(index);
        self.recursive_validator.validate(index);
        self.validate_configured_templates(index);

        // XXX: To avoid bloating up this function any further, maybe package logic into seperate module or
        //      function if another global check is introduced (including the overflow checks)?
        // Find and report const-expressions that would overflow
        for it in index.get_const_expressions().into_iter() {
            let Some(expr) = index.get_const_expressions().find_const_expression(&it.0) else { continue };
            if let ConstExpression::Unresolvable { reason, .. } = expr {
                if let UnresolvableKind::Overflow(reason, location) = reason.as_ref() {
                    self.push_diagnostic(
                        Diagnostic::new(reason).with_error_code("E039").with_location(location),
                    );
                }
            };
        }
    }

    pub fn visit_unit<T: AnnotationMap>(&mut self, annotations: &T, index: &Index, unit: &CompilationUnit) {
        let context =
            ValidationContext { annotations, index, qualifier: None, is_call: false, is_cast: false };
        // Validate POU and declared Variables
        for pou in &unit.pous {
            let context = context.with_qualifier(pou.name.as_str());

            visit_pou(self, pou, &context);
            visit_property(self, &context);
        }

        // Validate user declared types
        for t in &unit.user_types {
            visit_user_type_declaration(self, t, &context);
        }

        // Validate config variables (VAR_CONFIG)
        for variable in &unit.var_config {
            visit_config_variable(self, variable, &context);
        }

        // Validate global variables
        for gv in &unit.global_vars {
            visit_variable_block(self, None, gv, &context);
        }

        // Validate implementations
        for implementation in &unit.implementations {
            visit_implementation(self, implementation, &context);
        }

        for interface in &unit.interfaces {
            visit_interface(self, interface, &context);
        }
    }

    pub fn validate_configured_templates(&mut self, index: &Index) {
        // get config variables as string, along with their locations
        let config_variables =
            index.get_config_variables().iter().fold(FxHashMap::default(), |mut acc, var| {
                match ExpressionPath::try_from(var) {
                    Ok(p) => p.expand(index).into_iter().for_each(|p| {
                        acc.entry(p)
                            .and_modify(|entry: &mut Vec<SourceLocation>| entry.push(var.location.clone()))
                            .or_insert(vec![var.location.clone()]);
                    }),
                    Err(e) => {
                        self.diagnostics.extend(e);
                    }
                }
                acc
            });
        // check for ambiguously configured template-variables
        config_variables.values().for_each(|loc| {
            if loc.len() > 1 {
                self.diagnostics.push(
                    Diagnostic::new("Template variable configured multiple times")
                        .with_error_code("E108")
                        .with_location(&loc[0])
                        .with_secondary_locations(loc.iter().skip(1).cloned().collect()),
                )
            }
        });

        // collect all template-instances
        let instances = index
            .filter_instances(|entry, _| !entry.is_constant())
            .filter(|(_, entry)| {
                entry.get_hardware_binding().is_some_and(|opt| opt.access == DirectAccessType::Template)
            })
            .fold(vec![], |mut acc, (path, idxentry)| {
                let path = path.expand(index);
                let is_array = path.len() > 1;
                path.into_iter().for_each(|p| {
                    acc.push((p, (idxentry, is_array)));
                });
                acc
            });

        let mut incomplete_array_configurations = FxIndexSet::default();
        // validate if all template-instances are configured in VAR_CONFIG blocks
        for (segments, (val, is_array)) in instances {
            if !config_variables.contains_key(&segments) {
                if is_array {
                    incomplete_array_configurations.insert(&val.source_location);
                } else {
                    self.diagnostics.push(
                        Diagnostic::new("Template-variable must have a configuration")
                            .with_error_code("E107")
                            .with_location(&val.source_location),
                    );
                }
            }
        }

        // report arrays which have elements that are missing configuration
        incomplete_array_configurations.iter().for_each(|array_location| {
            self.diagnostics.push(
                Diagnostic::new("One or more template-elements in array have not been configured")
                    .with_location(*array_location)
                    .with_error_code("E107"),
            );
        });
    }
}
