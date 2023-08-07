use plc_ast::{
    ast::{AstStatement, Variable},
    literals::AstLiteral,
};

use crate::{diagnostics::Diagnostic, resolver::AnnotationMap, typesystem::DataType};

use super::{ValidationContext, Validator, Validators};

pub(super) enum Wrapper<'a> {
    Statement(&'a AstStatement),
    Variable(&'a Variable),
}

impl<'a> Wrapper<'a> {
    fn get_rhs(&self) -> Option<&'a AstStatement> {
        match self {
            Wrapper::Statement(AstStatement::Assignment { right, .. }) => Some(right),
            Wrapper::Variable(variable) => variable.initializer.as_ref(),
            _ => None,
        }
    }

    fn is_assignment(&self) -> bool {
        matches!(self, Wrapper::Variable(..) | Wrapper::Statement(AstStatement::Assignment { .. }))
    }

    fn datatype_lhs<T>(&self, context: &'a ValidationContext<T>) -> Option<&'a DataType>
    where
        T: AnnotationMap,
    {
        match self {
            Wrapper::Statement(statement) => {
                let AstStatement::Assignment { left, .. } = statement else { return None };
                context.annotations.get_type(left, context.index)
            }

            Wrapper::Variable(variable) => variable
                .data_type_declaration
                .get_referenced_type()
                .and_then(|it| context.index.find_effective_type_by_name(&it)),
        }
    }
}

pub(super) fn validate_array_assignment<T>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    wrapper: Wrapper,
) where
    T: AnnotationMap,
{
    if !wrapper.is_assignment() {
        return;
    }

    if let Some(l_dt) = wrapper.datatype_lhs(context) {
        let r = wrapper.get_rhs().unwrap();
        if l_dt.is_array() {
            if !(r.is_literal_array() || r.is_reference()) {
                validator.push_diagnostic(Diagnostic::array_invalid_assigment(r.get_location()));
            } else {
                // Only if there was no issue with assignment do we want to validate their sizes
                let len_lhs = l_dt.get_type_information().get_array_length(context.index).unwrap_or(0);
                let len_rhs = statement_to_array_length(r);

                if len_lhs < len_rhs {
                    let diagnostic =
                        Diagnostic::array_size(l_dt.get_name(), len_lhs, len_rhs, r.get_location());
                    validator.push_diagnostic(diagnostic);
                }
            }
        }
    }
}

/// Takes an [`AstStatement`] and returns its length as if it was an array. For example calling this function
/// on the expression-list of `foo := ((...), (...))` would return 2.
fn statement_to_array_length(statement: &AstStatement) -> usize {
    match statement {
        AstStatement::ExpressionList { .. } => 1,
        AstStatement::MultipliedStatement { multiplier, .. } => *multiplier as usize,
        AstStatement::Literal { kind: AstLiteral::Array(arr), .. } => match arr.elements() {
            Some(AstStatement::ExpressionList { expressions, .. }) => {
                expressions.iter().map(statement_to_array_length).sum::<usize>()
            }

            Some(any) => statement_to_array_length(any),
            None => 0,
        },

        // Any literal other than an array can be counted as 1
        AstStatement::Literal { .. } => 1,

        any => {
            // XXX: Not sure what else could be in here
            log::warn!("Array size counting for {any:?} not covered; validation _could_ be wrong");
            0
        }
    }
}
