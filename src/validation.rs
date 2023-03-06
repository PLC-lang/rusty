use crate::{
    ast::{AstStatement, CompilationUnit, SourceRange},
    index::{Index, PouIndexEntry, VariableIndexEntry},
    resolver::{AnnotationMap, AnnotationMapImpl},
    Diagnostic,
};

use self::{
    global_validator::GlobalValidator,
    pou::{validate_action_container, visit_pou},
    recursive_validator::RecursiveValidator,
    statement::visit_statement,
    types::visit_user_type_declaration,
    variable::visit_variable_block,
};

mod global_validator;
mod pou;
mod recursive_validator;
mod statement;
mod types;
mod variable;

#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct ValidationContext<'s> {
    annotations: &'s AnnotationMapImpl,
    index: &'s Index,
    /// the type_name of the context for a reference (e.g. `a.b` where `a`'s type is the context of `b`)
    qualifier: Option<&'s str>,
}

impl<'s> ValidationContext<'s> {
    fn with_qualifier(&self, qualifier: &'s str) -> ValidationContext<'s> {
        ValidationContext { annotations: self.annotations, index: self.index, qualifier: Some(qualifier) }
    }

    fn find_pou(&self, stmt: &AstStatement) -> Option<&PouIndexEntry> {
        match stmt {
            AstStatement::Reference { name, .. } => Some(name.as_str()),
            AstStatement::QualifiedReference { elements, .. } => {
                if let Some(name) = elements.last().and_then(|it| self.annotations.get_call_name(it)) {
                    Some(name)
                } else {
                    None
                }
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
}

/// This trait should be implemented by any validator used by `validation::Validator`
pub trait Validators {
    fn push_diagnostic(&mut self, diagnostic: Diagnostic);

    fn take_diagnostics(&mut self) -> Vec<Diagnostic>;
}

pub struct Validator {
    //context: ValidationContext<'s>,
    diagnostics: Vec<Diagnostic>,
    global_validator: GlobalValidator,
    recursive_validator: RecursiveValidator,
}

impl Validators for Validator {
    fn push_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn take_diagnostics(&mut self) -> Vec<Diagnostic> {
        std::mem::take(&mut self.diagnostics)
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
        self.global_validator.validate_unique_symbols(index);
        self.recursive_validator.validate_recursion(index);
    }

    pub fn visit_unit(&mut self, annotations: &AnnotationMapImpl, index: &Index, unit: &CompilationUnit) {
        let context = ValidationContext { annotations, index, qualifier: None };
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
            visit_variable_block(self, gv, &context);
        }

        // validate implementations
        for implementation in &unit.implementations {
            validate_action_container(self, implementation);
            implementation.statements.iter().for_each(|s| {
                visit_statement(self, s, &context.with_qualifier(implementation.name.as_str()))
            });
        }
    }

    fn validate_assignment_type_sizes(
        &mut self,
        idx_entry: &VariableIndexEntry,
        statement: &AstStatement,
        location: SourceRange,
        context: &ValidationContext,
    ) {
        let index = &context.index;
        let assigned_type = context.annotations.get_type(&statement, &index);
        let actual_type = context.annotations.get_type_hint(&statement, &index);
        let (Some(assigned_type), Some(actual_type)) = (assigned_type, actual_type) else {
            return
        };

        if assigned_type.get_type_information().get_size(&index)
            > actual_type.get_type_information().get_size(&index)
        {
            self.push_diagnostic(Diagnostic::implicit_downcast(
                actual_type.get_name(),
                assigned_type.get_name(),
                location,
            ))
        }
    }
}

/// Finds and reports invalid `ARRAY` assignments where parentheses are missing yielding invalid ASTs.
/// Specifically an invalid assignment such as `x := (var1 := 1, var2 := 3, 4);` where `var2` is missing a
/// `(` will generate `ExpressionList { Assignment {..}, ...}` as the AST where each item after
/// the first one would be handled as a seperate statement whereas the correct AST should have been
/// `Assignment { left: Reference {..}, right: ExpressionList {..}}`. See also
/// - https://github.com/PLC-lang/rusty/issues/707 and
/// - `array_validation_test.rs/array_initialization_validation`
pub fn validate_for_array_assignment(
    validator: &mut Validator,
    expressions: &[AstStatement],
    context: &ValidationContext,
) {
    let mut array_assignment = false;
    expressions.iter().for_each(|e| {
        if array_assignment {
            // now we cannot be sure where the following values belong to
            validator
                .push_diagnostic(Diagnostic::array_expected_identifier_or_round_bracket(e.get_location()));
        }
        match e {
            AstStatement::Assignment { left, right, .. } => {
                let left_type =
                    context.annotations.get_type_or_void(left, context.index).get_type_information();
                let right_type =
                    context.annotations.get_type_or_void(right, context.index).get_type_information();

                if left_type.is_array()
				// if we try to assign an `ExpressionList` to an ARRAY
				// we can expect that `()` were used and we got a valid parse result
				 && !matches!(right.as_ref(), AstStatement::ExpressionList { .. })
                 && !right_type.is_array()
                {
                    // otherwise we are definitely in an invalid assignment
                    array_assignment = true;
                    validator
                        .push_diagnostic(Diagnostic::array_expected_initializer_list(left.get_location()));
                }
            }
            AstStatement::ExpressionList { expressions, .. } => {
                // e.g. ARRAY OF STRUCT can have multiple `ExpressionList`s
                validate_for_array_assignment(validator, expressions, context);
            }
            _ => {} // do nothing
        }
    })
}
