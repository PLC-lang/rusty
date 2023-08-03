//! TODO: ...

use plc_ast::{ast::AstStatement, literals::AstLiteral};

use crate::{diagnostics::Diagnostic, resolver::AnnotationMap};

use super::{ValidationContext, Validator, Validators};

pub(super) fn validate<T>(validator: &mut Validator, context: &ValidationContext<T>, statement: &AstStatement)
where
    T: AnnotationMap,
{
    if statement.is_expression_list()
        | statement.is_assignment()
        | statement.is_literal_array()
        | statement.is_multiplied_statement()
    {
        validate_size(validator, context, statement);
    }
}

fn validate_size<T>(validator: &mut Validator, context: &ValidationContext<T>, statement: &AstStatement)
where
    T: AnnotationMap,
{
    if let Some(dt) = context.annotations.get_type_hint(statement, context.index) {
        let dti = dt.get_type_information();
        if dti.is_array() {
            let len_lhs = dti.get_array_length(context.index).unwrap_or(0);
            let len_rhs = statement_to_array_length(statement);

            if len_lhs < len_rhs {
                let diagnostic =
                    Diagnostic::array_size(dti.get_name(), len_lhs, len_rhs, statement.get_location());
                validator.push_diagnostic(diagnostic);
            }
        }
    }

    match statement {
        AstStatement::Assignment { right, .. } => validate_size(validator, context, &right),
        AstStatement::ExpressionList { expressions, .. } => {
            expressions.iter().for_each(|expression| validate_size(validator, context, expression));
        }
        _ => (),
    }
}

fn statement_to_array_length(statement: &AstStatement) -> usize {
    match statement {
        AstStatement::ExpressionList { expressions, .. } => expressions.len(),
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

        // TODO: Not sure what else could be in here
        any => {
            log::warn!("Array size counting for {any:?} not covered. Result might be wrong.");
            0
        }
    }
}
