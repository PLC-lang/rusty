use rusty_derive::Validators;

use crate::{
    ast::{AstStatement, CompilationUnit},
    index::{
        const_expressions::{ConstExpression, UnresolvableKind},
        Index, PouIndexEntry,
    },
    resolver::AnnotationMap,
    Diagnostic,
};

use self::{
    global::GlobalValidator,
    pou::{visit_implementation, visit_pou},
    recursive::RecursiveValidator,
    types::visit_user_type_declaration,
    variable::visit_variable_block,
};

mod array;
mod global;
mod pou;
mod recursive;
mod statement;
mod types;
mod variable;

#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct ValidationContext<'s, T: AnnotationMap> {
    annotations: &'s T,
    index: &'s Index,
    /// the type_name of the context for a reference (e.g. `a.b` where `a`'s type is the context of `b`)
    qualifier: Option<&'s str>,
    is_call: bool,
}

impl<'s, T: AnnotationMap> ValidationContext<'s, T> {
    fn with_qualifier(&self, qualifier: &'s str) -> Self {
        ValidationContext {
            annotations: self.annotations,
            index: self.index,
            qualifier: Some(qualifier),
            is_call: self.is_call,
        }
    }

    fn find_pou(&self, stmt: &AstStatement) -> Option<&PouIndexEntry> {
        match stmt {
            AstStatement::Reference { name, .. } => Some(name.as_str()),
            AstStatement::QualifiedReference { elements, .. } => {
                elements.last().and_then(|it| self.annotations.get_call_name(it))
            }
            _ => None,
        }
        .and_then(|pou_name| {
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
        ValidationContext {
            annotations: self.annotations,
            index: self.index,
            qualifier: self.qualifier,
            is_call: true,
        }
    }

    fn is_call(&self) -> bool {
        self.is_call
    }
}

/// This trait should be implemented by any validator used by `validation::Validator`
pub trait Validators {
    fn push_diagnostic(&mut self, diagnostic: Diagnostic);

    fn take_diagnostics(&mut self) -> Vec<Diagnostic>;
}

#[derive(Validators)]
pub struct Validator {
    //context: ValidationContext<'s>,
    diagnostics: Vec<Diagnostic>,
    global_validator: GlobalValidator,
    recursive_validator: RecursiveValidator,
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

impl Validator {
    pub fn new() -> Validator {
        Validator {
            diagnostics: Vec::new(),
            global_validator: GlobalValidator::new(),
            recursive_validator: RecursiveValidator::new(),
        }
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

        // XXX: To avoid bloating up this function any further, maybe package logic into seperate module or
        //      function if another global check is introduced (including the overflow checks)?
        // Find and report const-expressions that would overflow
        for it in index.get_const_expressions().into_iter() {
            let Some(expr) = index.get_const_expressions().find_const_expression(&it.0) else { continue };
            let ConstExpression::Unresolvable { reason: UnresolvableKind::Overflow(reason, location), .. } = expr else { continue };

            self.push_diagnostic(Diagnostic::overflow(reason.to_owned(), location.to_owned()));
        }
    }

    pub fn visit_unit<T: AnnotationMap>(&mut self, annotations: &T, index: &Index, unit: &CompilationUnit) {
        let context = ValidationContext { annotations, index, qualifier: None, is_call: false };
        // validate POU and declared Variables
        for pou in &unit.units {
            visit_pou(self, pou, &context.with_qualifier(pou.name.as_str()));
        }

        // validate user declared types
        for t in &unit.user_types {
            visit_user_type_declaration(self, t, &context);
        }

        // validate global variables
        for gv in &unit.global_vars {
            visit_variable_block(self, None, gv, &context);
        }

        // validate implementations
        for implementation in &unit.implementations {
            visit_implementation(self, implementation, &context);
        }
    }
}

// TODO: Remove this
pub fn validate_array_assignment<T: AnnotationMap>(
    validator: &mut Validator,
    expressions: &[AstStatement],
    context: &ValidationContext<T>,
) {
    for expression in expressions {
        match expression {
            AstStatement::Assignment { left, right, .. } => {
                let lt = context.annotations.get_type_or_void(left, context.index).get_type_information();
                let rt = context.annotations.get_type_or_void(right, context.index).get_type_information();

                // For initializers we expect either an array, an expression list (`arr := (1, 2, 3,...)`) or
                // a multiplied statement (`arr := 32(0)`), anything else we can assume to be incorrect
                if lt.is_array()
                    && !rt.is_array()
                    && !right.is_expression_list()
                    && !right.is_multiplied_statement()
                {
                    validator
                        .push_diagnostic(Diagnostic::array_expected_initializer_list(left.get_location()));
                }
            }

            // For example visit all expressions in `arr : ARRAY[...] OF myStruct := ((...), (...))`
            AstStatement::ExpressionList { expressions, .. } => {
                validate_array_assignment(validator, expressions, context);
            }

            _ => {}
        }
    }
}
