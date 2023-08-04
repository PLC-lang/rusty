use plc_ast::{
    ast::{AstStatement, Variable},
    literals::AstLiteral,
};

use crate::{diagnostics::Diagnostic, resolver::AnnotationMap, typesystem::DataType};

use super::{ValidationContext, Validator, Validators};

pub(super) fn validate_array_assignment<T>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    statement: &AstStatement,
    variable: Option<&Variable>,
) where
    T: AnnotationMap,
{
    // Two ways this function gets called,
    // 1) On variable initializations inside the variable block
    // 2) On variable assigments inside the body
    // For 1) we carry the `variable` field
    if !statement.is_assignment() {
        // TODO: 1)
        // return; // TODO:
    }

    let initializer_hint = variable
        .and_then(|it| it.data_type_declaration.get_referenced_type())
        .and_then(|it| context.index.find_effective_type_by_name(&it));

    validate(validator, context, statement, initializer_hint)
}

fn validate<T>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    statement: &AstStatement,
    hint: Option<&DataType>,
) where
    T: AnnotationMap,
{
    match statement {
        AstStatement::Assignment { left, right, .. } => validate(
            validator,
            context,
            &right,
            Some(context.annotations.get_type_or_void(&left, context.index)), // We have to give hint here, for cases such as arr := 1, 2, 3 (=> should have been [1, 2, 3])
        ),
        AstStatement::ExpressionList { expressions, .. } => {
            expressions.iter().for_each(|it| validate(validator, context, it, None))
        }
        _ => (),
    }

    // if let AstStatement::Assignment { left, right, .. } = statement {
    //     let lt = context.annotations.get_type_or_void(&left, &context.index);
    //     let rt = context.annotations.get_type_or_void(&left, &context.index);

    //     if lt.is_array() && (!rt.is_array() && !(right.is_literal_array() || right.is_multiplied_statement()))
    //     {
    //         validator.push_diagnostic(Diagnostic::array_invalid_assigment(right.get_location()));
    //     }
    // }

    if let Some(hint) = hint.or(context.annotations.get_type_hint(statement, context.index)) {
        if hint.is_array()
            && !(statement.is_literal_array()
                || statement.is_multiplied_statement()
                || statement.is_reference())
        {
            if !statement.is_reference() {
                validator.push_diagnostic(Diagnostic::array_invalid_assigment(statement.get_location()));
            }
        } else {
            // Only if there was no issue with assignment do we want to validate their sizes
            if hint.is_array() {
                let len_lhs = hint.get_type_information().get_array_length(context.index).unwrap_or(0);
                let len_rhs = statement_to_array_length(statement);

                if len_lhs < len_rhs {
                    let diagnostic =
                        Diagnostic::array_size(hint.get_name(), len_lhs, len_rhs, statement.get_location());
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

        any => {
            // XXX: Not sure what else could be in here
            log::warn!("Array size counting for {any:?} not covered; validation _could_ be wrong");
            0
        }
    }
}
