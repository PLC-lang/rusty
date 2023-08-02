//! TODO: ...

use plc_ast::{ast::AstStatement, literals::AstLiteral};

use crate::{diagnostics::Diagnostic, resolver::AnnotationMap};

use super::{ValidationContext, Validator, Validators};

pub(super) fn validate<T>(validator: &mut Validator, context: &ValidationContext<T>, statement: &AstStatement)
where
    T: AnnotationMap,
{
    _match(validator, context, statement)
}

fn _match<T>(validator: &mut Validator, context: &ValidationContext<T>, statement: &AstStatement)
where
    T: AnnotationMap,
{
    match statement {
        AstStatement::Literal { .. } => validate_size(validator, context, statement),
        AstStatement::ExpressionList { .. } => validate_size(validator, context, statement),
        AstStatement::Assignment { .. } => validate_size(validator, context, statement),
        _ => {
            dbg!(&statement);
        }
    }
}

fn validate_size<T>(validator: &mut Validator, context: &ValidationContext<T>, statement: &AstStatement)
where
    T: AnnotationMap,
{
    match statement {
        AstStatement::Assignment { right, .. } => validate_size(validator, context, &right),
        AstStatement::ExpressionList { expressions, .. } => {
            expressions.iter().for_each(|expression| validate_size(validator, context, expression));
        }
        _ => (),
    }

    let Some(dti) = context.annotations.get_type_hint(statement, context.index).map(|it| it.get_type_information()) else { return };
    if !dti.is_array() {
        return;
    }

    let len_lhs = dti.get_array_length(context.index).unwrap_or(0);
    let len_rhs = statement_to_array_length(statement);

    if len_lhs < len_rhs {
        let diagnostic = Diagnostic::array_size(dti.get_name(), len_lhs, len_rhs, statement.get_location());
        validator.push_diagnostic(diagnostic);
    }
}

fn statement_to_array_length(statement: &AstStatement) -> usize {
    match statement {
        AstStatement::ExpressionList { expressions, .. } => expressions.len(),
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
