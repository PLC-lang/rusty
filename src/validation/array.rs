//! This module is responsible for validating array assignments both in their syntax and semantics.
//!
//! Specifically this module checks if the array assignments start with a leading `[` symbol and the fed
//! elements are less-equal to the arrays size. As an example, `foo : ARRAY[1..2] OF DINT := (1, 2, 3)`
//! violates both the syntax and semantic of array assignments.
//!
//! Design note: Because we distinguish between variables inside VAR blocks [`plc_ast::ast::Variable`]
//! and POU bodies [`plc_ast::ast::AstStatementKind`] and how we interact with them (e.g. infering types of
//! [`plc_ast::ast::Variable`] from the AstAnnotation being impossible right now) a wrapper enum was
//! introduced to make the validation code as generic as possible.

use plc_ast::{
    ast::{AstNode, AstStatement, Variable},
    literals::AstLiteral,
};
use plc_diagnostics::diagnostics::Diagnostic;

use crate::{resolver::AnnotationMap, typesystem::DataTypeInformation};

use super::{ValidationContext, Validator, Validators};

/// Indicates whether an array was defined in a VAR block or a POU body
#[derive(Debug)]
pub(super) enum Wrapper<'a> {
    Statement(&'a AstNode),
    Variable(&'a Variable),
}

pub(super) fn validate_array_assignment<T: AnnotationMap>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    wrapper: Wrapper,
) {
    let Some(lhs_type) = wrapper.datatype_info_lhs(context) else {
        return;
    };
    let Some(rhs_stmt) = wrapper.get_rhs() else {
        return;
    };

    if !lhs_type.is_array() {
        return;
    }

    validate_array(validator, context, lhs_type, rhs_stmt);
    validate_array_of_structs(validator, context, lhs_type, rhs_stmt);
}

fn validate_array<T: AnnotationMap>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    lhs_type: &DataTypeInformation,
    rhs_stmt: &AstNode,
) {
    let stmt_rhs = peel(rhs_stmt);
    let stmt_rhs = peel(stmt_rhs);
    if !(stmt_rhs.is_literal_array() || stmt_rhs.is_reference()) {
        validator.push_diagnostic(Diagnostic::array_assignment(stmt_rhs.get_location()));
        return; // Return here, because array size validation is error-prone with incorrect assignments
    }

    let len_lhs = lhs_type.get_array_length(context.index).unwrap_or(0);
    let len_rhs = statement_to_array_length(stmt_rhs);

    if len_lhs < len_rhs {
        let name = lhs_type.get_name();
        let location = stmt_rhs.get_location();
        validator.push_diagnostic(Diagnostic::array_size(name, len_lhs, len_rhs, location));
    }
}

fn validate_array_of_structs<T: AnnotationMap>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    lhs_type: &DataTypeInformation,
    rhs_stmt: &AstNode,
) {
    let Some(array_type_name) = lhs_type.get_inner_array_type_name() else {
        return;
    };
    let Some(dti) = context.index.find_effective_type_by_name(array_type_name) else {
        return;
    };

    if !dti.is_struct() {
        return;
    }

    let AstStatement::Literal(AstLiteral::Array(array)) = rhs_stmt.get_stmt() else {
        return;
    };
    let Some(elements) = array.elements().map(AstNode::get_stmt) else {
        return;
    };

    match elements {
        AstStatement::ExpressionList(expressions) => {
            for invalid in expressions.iter().filter(|it| !it.is_paren()) {
                validator.push_diagnostic(Diagnostic::array_struct_assignment(invalid.get_location()));
            }
        }

        // arr := [foo := 0]
        AstStatement::Assignment(..) => {
            validator.push_diagnostic(Diagnostic::array_struct_assignment(rhs_stmt.get_location()));
        }

        _ => (),
    }
}

/// Takes an [`AstStatementKind`] and returns its length as if it was an array. For example calling this function
/// on an expression-list such as `[(...), (...)]` would return 2.
fn statement_to_array_length(statement: &AstNode) -> usize {
    match statement.get_stmt() {
        AstStatement::ExpressionList { .. } => 1,
        AstStatement::ParenExpression(_) => 1,
        AstStatement::MultipliedStatement(data) => data.multiplier as usize,
        AstStatement::Literal(AstLiteral::Array(arr)) => match arr.elements() {
            Some(AstNode { stmt: AstStatement::ExpressionList(expressions), .. }) => {
                expressions.iter().map(statement_to_array_length).sum::<usize>()
            }

            Some(any) => statement_to_array_length(any),
            None => 0,
        },

        // Any literal other than an array can be counted as 1
        AstStatement::Literal { .. } => 1,

        _any => {
            // XXX: Not sure what else could be in here
            log::warn!("Array size-counting for {statement:?} not covered; validation _might_ be wrong");
            0
        }
    }
}

impl<'a> Wrapper<'a> {
    fn get_rhs(&self) -> Option<&'a AstNode> {
        match self {
            Wrapper::Statement(AstNode { stmt: AstStatement::Assignment(data), .. }) => Some(&data.right),
            Wrapper::Variable(variable) => variable.initializer.as_ref(),
            _ => None,
        }
    }

    fn datatype_info_lhs<T>(&self, context: &'a ValidationContext<T>) -> Option<&'a DataTypeInformation>
    where
        T: AnnotationMap,
    {
        match self {
            Wrapper::Statement(statement) => {
                let AstNode { stmt: AstStatement::Assignment(data), .. } = statement else {
                    return None;
                };
                context.annotations.get_type(&data.left, context.index).map(|it| it.get_type_information())
            }

            Wrapper::Variable(variable) => variable
                .data_type_declaration
                .get_referenced_type()
                .and_then(|it| context.index.find_effective_type_info(&it)),
        }
    }
}

fn peel(node: &AstNode) -> &AstNode {
    match &node.stmt {
        AstStatement::ParenExpression(expr) => peel(expr),
        _ => node,
    }
}
