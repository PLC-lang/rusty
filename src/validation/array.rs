//! TODO: ...

use plc_ast::{
    ast::{AstStatement, DataTypeDeclaration, Variable},
    literals::AstLiteral,
};

use crate::{diagnostics::Diagnostic, resolver::AnnotationMap, typesystem::DataTypeInformation};

use super::{ValidationContext, Validator, Validators};

pub enum ValidationKind<'a> {
    Variable(&'a Variable),
    Statement(&'a AstStatement),
}

pub fn validate<T>(validator: &mut Validator, context: &ValidationContext<T>, kind: ValidationKind)
where
    T: AnnotationMap,
{
    match kind {
        ValidationKind::Variable(variable) => initialization(validator, context, variable),
        ValidationKind::Statement(statement) => assignment(validator, context, statement),
    }
}

/// Validation for array initializations, i.e. directly in the declaration within a VAR-Block
fn initialization<T>(validator: &mut Validator, context: &ValidationContext<T>, variable: &Variable)
where
    T: AnnotationMap,
{
    let Some(initializer) = &variable.initializer else { return };
    if context.annotations.get_hint_or_void(initializer, context.index).is_array() {
        let DataTypeDeclaration::DataTypeReference { referenced_type, .. } = &variable.data_type_declaration else { todo!("definition?") };
        let Some(ldt) = context.index.find_effective_type_by_name(referenced_type).map(|it| it.get_type_information()) else { return };

        array_size(context, ldt, initializer, validator);
    } else {
        assignment(validator, context, initializer)
    }
}

/// Validation for array assignments
fn assignment<T>(validator: &mut Validator, context: &ValidationContext<T>, statement: &AstStatement)
where
    T: AnnotationMap,
{
    // foo := [1, 2, 3, 4, 5, 6]; // ARRAY[1..5] OF DINT;
    // ^^^^^^^^^^^^^^^^^^^^^^^^^
    //        We get this

    match statement {
        AstStatement::Assignment { left, right, .. } => {
            if !context.annotations.get_hint_or_void(right, context.index).is_array() {
                return; // We're not really interested if the rhs isn't an array
            }

            let Some(ldt) = context.annotations.get_type(left, context.index).map(|it| it.get_type_information()) else { return; };
            array_size(context, ldt, right, validator);
        }

        AstStatement::ExpressionList { expressions, .. } => {
            for expression in expressions {
                assignment(validator, context, expression);
            }
        }

        AstStatement::Literal { .. } => (),

        _ => (),
    }
}

fn array_size<T>(
    context: &ValidationContext<T>,
    left: &DataTypeInformation,
    right: &AstStatement,
    validator: &mut Validator,
) where
    T: AnnotationMap,
{
    let len_lhs = left.get_array_length(context.index).unwrap_or(0);
    let len_rhs = statement_to_array_length(right);

    if len_lhs < len_rhs {
        let diagnostic = Diagnostic::array_size(left.get_name(), len_lhs, len_rhs, right.get_location());
        validator.push_diagnostic(diagnostic)
    }

    // Visit each expression
    if let AstStatement::ExpressionList { expressions, .. } = right {
        for expression in expressions {
            assignment(validator, context, expression);
        }
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
