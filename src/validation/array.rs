//! TODO: ...

use plc_ast::{ast::AstStatement, literals::AstLiteral};

use crate::{
    diagnostics::{Diagnostic, ErrNo},
    resolver::AnnotationMap,
};

use super::{ValidationContext, Validator, Validators};

pub fn validate_array_assignment<T>(
    validator: &mut Validator,
    statement: &AstStatement,
    context: &ValidationContext<T>,
) where
    T: AnnotationMap,
{
    // foo := [1, 2, 3, 4, 5, 6]; // ARRAY[1..5] OF DINT;
    // ^^^^^^^^^^^^^^^^^^^^^^^^^
    //        We get this

    match statement {
        AstStatement::Assignment { left, right, .. } => {
            if !context.annotations.get_hint_or_void(&right, context.index).is_array() {
                return; // We're not really interested if the rhs isn't an array
            }

            let Some(ldt) = context.annotations.get_type(&left, context.index).map(|it| it.get_type_information()) else { return; };
            let lhs_len = ldt.get_array_length(context.index).unwrap_or(0);
            let rhs_len = statement_to_array_length(&right);

            println!("Length of lhs: {lhs_len}");
            println!("Length of rhs: {rhs_len}");

            if lhs_len < rhs_len {
                validator.push_diagnostic(Diagnostic::SemanticError {
                    message: format!("Array TODO has size {lhs_len}, but {rhs_len} were provided"),
                    range: vec![right.get_location()],
                    err_no: ErrNo::arr__invalid_array_assignment,
                })
            }
        }

        AstStatement::ExpressionList { expressions, .. } => {
            todo!();
            for expression in expressions {
                validate_array_assignment(validator, expression, context);
            }
        }
        _ => todo!(),
    }
}

fn statement_to_array_length(statement: &AstStatement) -> usize {
    match statement {
        AstStatement::ExpressionList { expressions, .. } => expressions.len(),
        AstStatement::Literal { kind, .. } => match kind {
            AstLiteral::Array(arr) => match arr.elements() {
                Some(statement) => match statement {
                    AstStatement::ExpressionList { expressions, .. } => {
                        // TODO: Explain why; nested arrays e.g. `[[1, 2, 3], [4, 5, 6]]`
                        expressions.iter().map(|it| statement_to_array_length(it)).sum::<usize>()
                    }
                    _ => statement_to_array_length(statement),
                },
                None => 0,
            },

            // Any other literal counts as 1
            _ => 1,
        },

        // TODO: Not sure what else could be in here
        _ => 0,
    }
}
