use plc_ast::{ast::AstStatement, literals::AstLiteral};

use crate::{diagnostics::Diagnostic, resolver::AnnotationMap};

use super::{ValidationContext, Validator, Validators};

pub(super) fn validate_array_assignment<T>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    statement: &AstStatement,
) where
    T: AnnotationMap,
{
    if statement.is_expression_list()
        | statement.is_assignment()
        | statement.is_literal_array()
        | statement.is_multiplied_statement()
    {
        validate_size(validator, context, statement);
        validate_assignment(validator, context, statement);
    }
}

/// Validates if an array assignment is valid in regard to their size.
fn validate_size<T>(validator: &mut Validator, context: &ValidationContext<T>, statement: &AstStatement)
where
    T: AnnotationMap,
{
    // If we have a type-hint on an AST statement, check if it's an array and if so validate it's size.
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

    // Regardless of whether or not it was an array, check if we may be able to find further validation cases
    // by checking inner expressions. For example here `foo := ((bar := [...]), (bar := [...]))` might be a
    // struct with array fields `bar`, which we have to validate. The previous check wouldn't have caught them.
    match statement {
        AstStatement::Assignment { right, .. } => validate_size(validator, context, &right),
        AstStatement::ExpressionList { expressions, .. } => {
            expressions.iter().for_each(|expression| validate_size(validator, context, expression));
        }
        _ => (),
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

// TODO(volsa): Refactor this, maybe with issue https://github.com/PLC-lang/rusty/issues/707
/// Finds and reports invalid `ARRAY` assignments where parentheses are missing yielding invalid ASTs.
/// Specifically an invalid assignment such as `x := (var1 := 1, var2 := 3, 4);` where `var2` is missing a
/// `(` will generate `ExpressionList { Assignment {..}, ...}` as the AST where each item after
/// the first one would be handled as a seperate statement whereas the correct AST should have been
/// `Assignment { left: Reference {..}, right: ExpressionList {..}}`. See also
/// - https://github.com/PLC-lang/rusty/issues/707 and
/// - `array_validation_test.rs/array_initialization_validation`
pub fn validate_assignment<T: AnnotationMap>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    statement: &AstStatement,
) {
    if let AstStatement::ExpressionList { expressions, .. } = statement {
        for expression in expressions {
            validate_assignment(validator, context, expression);
        }
    }
    if let AstStatement::Assignment { left, right, .. } = statement {
        let Some(dti) = context.annotations.get_type(&left, context.index).map(|it| it.get_type_information()) else { return };
        if dti.is_array() {
            match dbg!(right.as_ref()) {
                AstStatement::Literal { kind, .. } => match kind {
                    AstLiteral::Array(..) => (),
                    _ => validator.push_diagnostic(Diagnostic::array_expected_identifier_or_round_bracket(
                        right.get_location(),
                    )),
                },
                _ => println!("{statement:?}"),
            }
        }
    }

    // let mut array_assignment = false;
    // expressions.iter().for_each(|e| {
    //     if array_assignment {
    //         // now we cannot be sure where the following values belong to
    //         validator
    //             .push_diagnostic(Diagnostic::array_expected_identifier_or_round_bracket(e.get_location()));
    //     }
    //     match e {
    //         AstStatement::Assignment { left, right, .. } => {
    //             let lt = context.annotations.get_type_or_void(left, context.index).get_type_information();
    //             let rt = context.annotations.get_type_or_void(right, context.index).get_type_information();

    //             // For initializers we expect either an array, an expression list (`arr := (1, 2, 3,...)`) or
    //             // a multiplied statement (`arr := 32(0)`), anything else we can assume to be incorrect
    //             if lt.is_array()
    //                 && !rt.is_array()
    //                 && !right.is_expression_list()
    //                 && !right.is_multiplied_statement()
    //             {
    //                 array_assignment = true;
    //                 validator
    //                     .push_diagnostic(Diagnostic::array_expected_initializer_list(left.get_location()));
    //             }
    //         }
    //         AstStatement::ExpressionList { expressions, .. } => {
    //             // e.g. ARRAY OF STRUCT can have multiple `ExpressionList`s
    //             validate_for_array_assignment(validator, expressions, context);
    //         }
    //         _ => {} // do nothing
    //     }
    // })
}
