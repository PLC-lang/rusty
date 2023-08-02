//! TODO: ...

use plc_ast::{
    ast::{AstStatement, DataTypeDeclaration, Variable},
    literals::AstLiteral,
};

use crate::{
    diagnostics::{Diagnostic, ErrNo},
    resolver::AnnotationMap,
};

use super::{ValidationContext, Validator, Validators};

pub fn validate_array_initialization<T>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    variable: &Variable,
) where
    T: AnnotationMap,
{
    let Some(initializer) = &variable.initializer else { return };
    if context.annotations.get_hint_or_void(initializer, context.index).is_array() {
        let DataTypeDeclaration::DataTypeReference { referenced_type, .. } = &variable.data_type_declaration else { todo!("definition?") };
        let Some(ldt) = context.index.find_effective_type_by_name(&referenced_type).map(|it| it.get_type_information()) else { return };

        let lhs_len = ldt.get_array_length(context.index).unwrap_or(0);
        let rhs_len = statement_to_array_length(initializer);

        println!("Length of lhs: {lhs_len}");
        println!("Length of rhs: {rhs_len}");

        if lhs_len < rhs_len {
            validator.push_diagnostic(Diagnostic::SemanticError {
                message: format!("Array TODO has size {lhs_len}, but {rhs_len} were provided"),
                range: vec![initializer.get_location()],
                err_no: ErrNo::arr__invalid_array_assignment,
            })
        }
    }

    if let AstStatement::ExpressionList { expressions, .. } = initializer {
        for expression in expressions {
            validate_array_assignment(validator, context, expression);
        }
    }
}

pub fn validate_array_assignment<T>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    statement: &AstStatement,
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

            // Visit each expression
            if let AstStatement::ExpressionList { expressions, .. } = right.as_ref() {
                for expression in expressions {
                    validate_array_assignment(validator, context, expression);
                }
            }
        }

        AstStatement::ExpressionList { expressions, .. } => {
            for expression in expressions {
                validate_array_assignment(validator, context, expression);
            }
        }

        AstStatement::Literal { .. } => (),

        _ => todo!("{statement:?}"),
    }
}

fn statement_to_array_length(statement: &AstStatement) -> usize {
    match statement {
        AstStatement::ExpressionList { expressions, .. } => expressions.len(),
        AstStatement::Literal { kind: AstLiteral::Array(arr), .. } => match arr.elements() {
            Some(AstStatement::ExpressionList { expressions, .. }) => {
                expressions.iter().map(|it| statement_to_array_length(it)).sum::<usize>()
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
