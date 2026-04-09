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
    ast::{AstNode, AstStatement, UserTypeDeclaration, Variable},
    literals::AstLiteral,
};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_index::GlobalContext;

use crate::{resolver::AnnotationMap, typesystem::DataTypeInformation};

use super::{ValidationContext, Validator, Validators};

/// Indicates whether an array was assigned in a VAR block, a POU body, or a TYPE declaration
#[derive(Debug, Clone, Copy)]
pub(super) enum StatementWrapper<'a> {
    Statement(&'a AstNode),
    Variable(&'a Variable),
    TypeDeclaration(&'a UserTypeDeclaration),
}

impl<'a> From<&'a AstNode> for StatementWrapper<'a> {
    fn from(value: &'a AstNode) -> Self {
        Self::Statement(value)
    }
}

impl<'a> From<&'a Variable> for StatementWrapper<'a> {
    fn from(value: &'a Variable) -> Self {
        Self::Variable(value)
    }
}

impl<'a> From<&'a UserTypeDeclaration> for StatementWrapper<'a> {
    fn from(value: &'a UserTypeDeclaration) -> Self {
        Self::TypeDeclaration(value)
    }
}

pub(super) fn validate_array_assignment<'a, T, S>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    statement: S,
) where
    T: AnnotationMap,
    S: Into<StatementWrapper<'a>> + Copy,
{
    let statement = statement.into();

    let Some(rhs_stmt) = statement.rhs_statement() else { return };
    let Some(lhs_info) = statement.lhs_info(context) else { return };

    if !lhs_info.is_array() {
        return;
    }

    validate_array(validator, context, &statement, lhs_info, rhs_stmt);
    validate_array_of_structs(validator, context, lhs_info, rhs_stmt);
}

fn validate_array<T: AnnotationMap>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    statement: &StatementWrapper,
    lhs_type: &DataTypeInformation,
    rhs_stmt: &AstNode,
) {
    let stmt_rhs = peel(rhs_stmt);
    if !(stmt_rhs.is_literal_array() || stmt_rhs.is_reference() || stmt_rhs.is_call()) {
        validator.push_diagnostic(
            Diagnostic::new("Array assignments must be surrounded with `[]`")
                .with_error_code("E043")
                .with_location(stmt_rhs),
        );
        return; // Return here, because array size validation is error-prone with incorrect assignments
    }

    let len_lhs = lhs_type.get_array_length(context.index).unwrap_or(0);
    let Some(len_rhs) = statement_to_array_length(context, stmt_rhs) else { return };

    if len_lhs == 0 {
        return;
    }

    if len_lhs < len_rhs {
        let name = statement.lhs_name(validator.context);
        let location = stmt_rhs.get_location();
        validator.push_diagnostic(
            Diagnostic::new(format!(
                "Too many initial values for array `{name}`. Expected {len_lhs}, found {len_rhs}."
            ))
            .with_error_code("E043")
            .with_location(location),
        );
    } else if len_rhs < len_lhs {
        let name = statement.lhs_name(validator.context);
        let location = stmt_rhs.get_location();
        validator.push_diagnostic(
            Diagnostic::new(format!(
                "Fewer initial values for array `{name}` than expected. Expected {len_lhs}, found {len_rhs}."
            ))
            .with_error_code("E127")
            .with_location(location),
        );
    }
}

/// Checks if an expression is a valid element in an array of structs.
/// Valid elements are:
/// - Parenthesized expressions (struct initializers like `(a := 1, b := 2)`)
/// - Reference expressions (variable references like `str`)
fn is_valid_struct_array_element(node: &AstNode) -> bool {
    matches!(node.get_stmt(), AstStatement::ParenExpression(_) | AstStatement::ReferenceExpr(_))
}

fn validate_array_of_structs<T: AnnotationMap>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    lhs_type: &DataTypeInformation,
    rhs_stmt: &AstNode,
) {
    let Some(array_type_name) = lhs_type.get_inner_array_type_name() else { return };
    let Some(dti) = context.index.find_effective_type_by_name(array_type_name) else { return };
    if !dti.is_struct() {
        return;
    }

    let AstStatement::Literal(AstLiteral::Array(array)) = rhs_stmt.get_stmt() else { return };
    let Some(elements) = array.elements().map(AstNode::get_stmt) else { return };

    match elements {
        AstStatement::ExpressionList(expressions) => {
            for invalid in expressions.iter().filter(|it| !is_valid_struct_array_element(it)) {
                validator.push_diagnostic(
                    Diagnostic::new("Struct initializers within arrays have to be wrapped by `()`")
                        .with_error_code("E043")
                        .with_location(invalid),
                );
            }
        }

        // arr := [foo := 0]
        AstStatement::Assignment(..) => {
            validator.push_diagnostic(
                Diagnostic::new("Struct initializers within arrays have to be wrapped by `()`")
                    .with_error_code("E043")
                    .with_location(rhs_stmt),
            );
        }

        _ => (),
    }
}

/// Takes an [`AstStatementKind`] and returns its length as if it was an array. For example calling this function
/// on an expression-list such as `[(...), (...)]` would return 2. Returns `None` if the length could not be
/// determined (e.g. for unresolved calls or unknown AST nodes), signaling that size validation should be skipped.
fn statement_to_array_length<T: AnnotationMap>(
    context: &ValidationContext<T>,
    statement: &AstNode,
) -> Option<usize> {
    match statement.get_stmt() {
        AstStatement::Literal(AstLiteral::Array(arr)) => match arr.elements() {
            Some(AstNode { stmt: AstStatement::ExpressionList(expressions), .. }) => {
                let mut total = 0;
                for it in expressions {
                    total += array_element_length(context, it)?;
                }
                Some(total)
            }

            Some(any) => array_element_length(context, any),
            None => Some(0),
        },

        AstStatement::CallStatement(_) => context
            .annotations
            .get_type(statement, context.index)
            .and_then(|it| it.information.get_array_length(context.index)),

        // At the top level, a reference to a variable is not an array initializer — skip
        // validation. References inside array literals are handled by `array_element_length`.
        AstStatement::ReferenceExpr(_) => None,

        AstStatement::MultipliedStatement(data) => Some(data.multiplier as usize),
        AstStatement::ExpressionList { .. } | AstStatement::ParenExpression(_) => Some(1),

        // Any literal other than an array can be counted as 1
        AstStatement::Literal { .. } => Some(1),

        _any => {
            log::debug!("Array size-counting for {statement:?} not covered; skipping validation");
            None
        }
    }
}

/// Returns the length of an element inside an array literal.
/// Unlike [`statement_to_array_length`], non-array references are counted as 1 element
/// since they represent individual values in the initializer list.
fn array_element_length<T: AnnotationMap>(
    context: &ValidationContext<T>,
    statement: &AstNode,
) -> Option<usize> {
    match statement.get_stmt() {
        AstStatement::ReferenceExpr(_) => {
            let len = context
                .annotations
                .get_type(statement, context.index)
                .and_then(|it| it.information.get_array_length(context.index));
            Some(len.unwrap_or(1))
        }
        _ => statement_to_array_length(context, statement),
    }
}

impl<'a> StatementWrapper<'a> {
    fn lhs_name(&self, context: &GlobalContext) -> String {
        match self {
            StatementWrapper::Variable(variable) => variable.name.clone(),
            StatementWrapper::Statement(statement) => {
                let AstStatement::Assignment(data) = &statement.stmt else { return "".to_string() };
                context.slice(&data.left.location)
            }
            StatementWrapper::TypeDeclaration(utd) => {
                utd.data_type.get_name().map(|s| s.to_string()).unwrap_or_default()
            }
        }
    }

    fn rhs_statement(&self) -> Option<&'a AstNode> {
        match self {
            StatementWrapper::Variable(variable) => variable.initializer.as_ref(),
            StatementWrapper::Statement(statement) => {
                let AstStatement::Assignment(data) = &statement.stmt else { return None };
                Some(&data.right)
            }
            StatementWrapper::TypeDeclaration(utd) => utd.initializer.as_ref(),
        }
    }

    fn lhs_info<T>(&self, context: &'a ValidationContext<T>) -> Option<&'a DataTypeInformation>
    where
        T: AnnotationMap,
    {
        let ty = match self {
            StatementWrapper::Statement(statement) => {
                let AstNode { stmt: AstStatement::Assignment(data), .. } = statement else { return None };
                context.annotations.get_type(&data.left, context.index).map(|it| it.get_type_information())
            }

            StatementWrapper::Variable(variable) => variable
                .data_type_declaration
                .get_referenced_type()
                .and_then(|it| context.index.find_effective_type_info(it)),

            StatementWrapper::TypeDeclaration(utd) => {
                utd.data_type.get_name().and_then(|name| context.index.find_effective_type_info(name))
            }
        }?;

        match ty {
            DataTypeInformation::Pointer { .. } => Some(context.index.find_elementary_pointer_type(ty)),
            _ => Some(ty),
        }
    }
}

fn peel(node: &AstNode) -> &AstNode {
    match &node.stmt {
        AstStatement::ParenExpression(expr) => peel(expr),
        _ => node,
    }
}
