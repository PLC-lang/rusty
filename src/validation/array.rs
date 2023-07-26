use crate::{
    ast::{AstLiteral, AstStatement},
    diagnostics::Diagnostic,
    resolver::AnnotationMap,
    typesystem::DataTypeInformation,
};

use super::{ValidationContext, Validator, Validators};

pub fn __validate_array_assignment<T: AnnotationMap>(
    validator: &mut Validator,
    statement: &AstStatement,
    context: &ValidationContext<T>,
) {
    array_size(validator, statement, context);
    assignment(validator, statement, context);
}

pub fn assignment<T: AnnotationMap>(
    validator: &mut Validator,
    expressions: &AstStatement,
    context: &ValidationContext<T>,
) {
    let AstStatement::ExpressionList { expressions, ..} = expressions else { return };
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
            AstStatement::ExpressionList { .. } => {
                // TODO: Unsure if this works
                __validate_array_assignment(validator, expression, context);
            }

            _ => {}
        }
    }
}

pub fn array_size<T: AnnotationMap>(
    validator: &mut Validator,
    statement: &AstStatement,
    context: &ValidationContext<T>,
) {
    let AstStatement::Assignment { left, right, .. } = statement else { return };
    let Some(lt) = context.annotations.get_type(&left, context.index) else { return };
    let DataTypeInformation::Array { dimensions, .. } = lt.get_type_information() else { return };
    let l_arr_len = dimensions[0].get_length(context.index).unwrap_or(0) as usize;

    let r_arr_len = match right.as_ref() {
        AstStatement::ExpressionList { expressions, .. } => expressions.len(),
        AstStatement::Literal { kind: AstLiteral::Array(arr), .. } => match arr.elements() {
            Some(AstStatement::ExpressionList { expressions, .. }) => expressions.len(),
            Some(AstStatement::Literal { kind: AstLiteral::Integer(_), .. }) => 1,
            _ => 0,
        },
        _ => 0,
    };

    if l_arr_len < r_arr_len {
        validator.push_diagnostic(Diagnostic::SemanticError {
            message: format!(
                "Array {name} has a size of {l_arr_len} but {r_arr_len} elements were provided",
                name = left.get_name().unwrap_or_default()
            ),
            range: vec![right.get_location()],
            err_no: crate::diagnostics::ErrNo::arr__invalid_array_assignment,
        });
    }
}
