use plc_ast::{
    ast::{
        flatten_expression_list, AstId, AstNode, AstStatement, BinaryExpression, CallStatement, Operator,
        ReferenceAccess, ReferenceExpr, UnaryExpression,
    },
    literals::AstLiteral,
};
use plc_diagnostics::diagnostics::Diagnostic;

use super::{ValidationContext, Validator, Validators};
use crate::{
    index::Index,
    resolver::{AnnotationMap, StatementAnnotation},
    typesystem::{get_literal_actual_signed_type_name, DataType, DataTypeInformation, VOID_TYPE},
};

/// How an implicit conversion can change a value
enum Conversion {
    SignChange,
    IntToFloat,
    FloatToInt,
    Time,
    Bool,
}

/// Where a conversion happens: the value of an expression is converted to the type it is assigned to, or
/// an operand is converted to the type that the operation works in
#[derive(Clone, Copy, PartialEq)]
enum Site {
    Assignment,
    Operand,
}

impl Conversion {
    fn code(&self, site: Site) -> &'static str {
        match (self, site) {
            (Conversion::SignChange, Site::Assignment) => "E160",
            (Conversion::SignChange, Site::Operand) => "E161",
            (Conversion::IntToFloat, _) => "E162",
            (Conversion::FloatToInt, _) => "E163",
            (Conversion::Time, _) => "E164",
            (Conversion::Bool, _) => "E165",
        }
    }

    fn message(&self, site: Site, source: &str, target: &str) -> String {
        match (self, site) {
            (Conversion::SignChange, Site::Assignment) => {
                format!("Implicit conversion from '{source}' to '{target}' may change the sign.")
            }
            (Conversion::SignChange, Site::Operand) => {
                format!("Operand of type '{source}' is converted to '{target}', which may change its sign.")
            }
            (Conversion::IntToFloat, _) => {
                format!("Implicit conversion from '{source}' to '{target}' may lose precision.")
            }
            (Conversion::FloatToInt, _) => {
                format!("Implicit conversion from '{source}' to '{target}' truncates the value.")
            }
            (Conversion::Time, Site::Assignment) => {
                format!("Implicit conversion from '{source}' to '{target}' mixes time or date types with other types.")
            }
            (Conversion::Time, Site::Operand) => {
                format!("Operand of type '{source}' is used with '{target}', which mixes time or date types with other types.")
            }
            (Conversion::Bool, _) => {
                format!("Implicit conversion from '{source}' to '{target}' mixes BOOL with a numeric type.")
            }
        }
    }
}

/// Reports the value-changing implicit conversions from the value of `expression` into `target`
pub(super) fn validate_assignment_conversions<T: AnnotationMap>(
    validator: &mut Validator,
    target: &DataType,
    expression: &AstNode,
    context: &ValidationContext<T>,
) {
    if !context.skips_argument_conversions() {
        validate_expression_conversions(validator, target, expression, false, context);
    }
}

/// Whether the arguments of the call are not converted to its parameters: a builtin operator call, such as
/// `ADD(a, b)`, is evaluated as its operation `a + b`, and the shift count of a bit shift counts bits
pub(super) fn skips_argument_conversions<T: AnnotationMap>(
    call: &AstNode,
    context: &ValidationContext<T>,
) -> bool {
    let AstStatement::CallStatement(CallStatement { operator, .. }) = call.get_stmt() else {
        return false;
    };

    // generic functions are called by the name of their implementation, such as `ROL__BYTE`
    let name = operator.get_flat_reference_name().unwrap_or_default();
    let generic_name = name.split("__").next().unwrap_or_default();
    operation_of(call, context).is_some() || matches!(generic_name, "SHL" | "SHR" | "ROL" | "ROR")
}

/// Reports the operands of the operation that a builtin operator call is evaluated as, like the operands of
/// the operator. The arguments are validated where they appear in the call.
pub(super) fn validate_operator_call_conversions<T: AnnotationMap>(
    validator: &mut Validator,
    call: &AstNode,
    context: &ValidationContext<T>,
) {
    let (Some(operation), AstStatement::CallStatement(CallStatement { parameters, .. })) =
        (operation_of(call, context), call.get_stmt())
    else {
        return;
    };

    let arguments: Vec<AstId> = parameters
        .as_deref()
        .map(flatten_expression_list)
        .unwrap_or_default()
        .into_iter()
        .map(|argument| match argument.get_stmt() {
            AstStatement::Assignment(assignment) => assignment.right.get_id(),
            _ => argument.get_id(),
        })
        .collect();
    validate_operation_operands(validator, operation, &arguments, context);
}

fn validate_operation_operands<T: AnnotationMap>(
    validator: &mut Validator,
    node: &AstNode,
    arguments: &[AstId],
    context: &ValidationContext<T>,
) {
    if arguments.contains(&node.get_id()) {
        return;
    }

    if let AstStatement::BinaryExpression(BinaryExpression { operator, left, right }) = node.get_stmt() {
        validate_operand_conversions(validator, operator, left, right, context);
        validate_operation_operands(validator, left, arguments, context);
        validate_operation_operands(validator, right, arguments, context);
    }
}

/// The binary expression that a builtin operator call, such as `ADD(a, b)`, is evaluated as
fn operation_of<'s, T: AnnotationMap>(
    call: &AstNode,
    context: &ValidationContext<'s, T>,
) -> Option<&'s AstNode> {
    match context.annotations.get(call)? {
        StatementAnnotation::ReplacementAst { statement }
            if matches!(statement.get_stmt(), AstStatement::BinaryExpression(_)) =>
        {
            Some(statement)
        }
        _ => None,
    }
}

fn validate_expression_conversions<T: AnnotationMap>(
    validator: &mut Validator,
    target: &DataType,
    expression: &AstNode,
    is_builtin_argument: bool,
    context: &ValidationContext<T>,
) {
    // a builtin operator call is converted like the operation it is evaluated as
    let operation = operation_of(peel(expression), context).unwrap_or(expression);

    match operation.get_stmt_peeled() {
        // untyped literals have the type their value needs, so only the value counts
        _ if untyped_integer_literal(expression).is_some() => {
            let value = untyped_integer_literal(expression).unwrap_or_default();
            validate_literal_conversion(validator, target, value, expression, Site::Assignment, context);
        }

        // an untyped literal takes the type of the other operand, so only its sign counts
        AstStatement::BinaryExpression(BinaryExpression { operator, left, right })
            if !operator.is_comparison_operator()
                && (is_literal_operand(left) || is_literal_operand(right))
                && !is_date_or_time(left, context)
                && !is_date_or_time(right, context) =>
        {
            for operand in [left, right] {
                match untyped_integer_literal(operand) {
                    Some(value) => {
                        validate_literal_sign(validator, target, value, operand, Site::Assignment, context)
                    }
                    None => validate_expression_conversions(validator, target, operand, false, context),
                }
            }
        }

        // the selector of SEL and MUX is never assigned to the target
        AstStatement::CallStatement(CallStatement { operator, parameters: Some(parameters) })
            if matches!(operator.get_flat_reference_name().unwrap_or_default(), "MUX" | "SEL") =>
        {
            if let AstStatement::ExpressionList(arguments) = parameters.get_stmt_peeled() {
                for argument in arguments.iter().skip(1) {
                    validate_expression_conversions(validator, target, argument, true, context);
                }
            }
        }

        // arguments of generic functions are converted to the type that the call resolves to
        _ if !is_builtin_argument && context.annotations.get_generic_nature(expression).is_some() => {}

        _ => validate_typed_conversion(validator, target, operation, expression, Site::Assignment, context),
    }
}

/// Reports the operands that the resolver converts to the type of the operation
pub(super) fn validate_operand_conversions<T: AnnotationMap>(
    validator: &mut Validator,
    operator: &Operator,
    left: &AstNode,
    right: &AstNode,
    context: &ValidationContext<T>,
) {
    if is_date_or_time(left, context) || is_date_or_time(right, context) {
        validate_date_or_time_operands(validator, operator, left, right, context);
        return;
    }

    // comparing addresses does not convert a value
    if is_pointer(left, context) || is_pointer(right, context) {
        return;
    }

    for operand in [left, right] {
        let Some(target) = context.annotations.get_type_hint(operand, context.index) else {
            continue;
        };

        match untyped_integer_literal(operand) {
            Some(value) => {
                validate_literal_conversion(validator, target, value, operand, Site::Operand, context)
            }
            None => validate_typed_conversion(validator, target, operand, operand, Site::Operand, context),
        }
    }
}

/// Reports a time or date operand used with an operand of another type
fn validate_date_or_time_operands<T: AnnotationMap>(
    validator: &mut Validator,
    operator: &Operator,
    left: &AstNode,
    right: &AstNode,
    context: &ValidationContext<T>,
) {
    match operator {
        // a number added to a duration is reported by its own rules, a duration is scaled by any number
        Operator::Plus | Operator::Minus | Operator::Multiplication | Operator::Division => {}
        _ => {
            let intrinsic_type = |node: &AstNode| {
                context
                    .annotations
                    .get_type(node, context.index)
                    .map(|ty| context.index.get_intrinsic_type_by_name(ty.get_name()))
            };
            let (Some(left_type), Some(right_type)) = (intrinsic_type(left), intrinsic_type(right)) else {
                return;
            };
            if left_type.get_name() == right_type.get_name() {
                return;
            }

            // the operand that is not a time or date, or the right one when both are
            let (operand, operand_type, other_type) =
                if is_date_or_time(right, context) && !is_date_or_time(left, context) {
                    (left, left_type, right_type)
                } else {
                    (right, right_type, left_type)
                };
            if is_scalar(operand_type.get_type_information()) {
                let source = match untyped_integer_literal(operand) {
                    Some(value) => literal_type_name(value),
                    None => validator.get_type_name_or_slice(operand_type),
                };
                report(validator, &Conversion::Time, Site::Operand, &source, other_type, operand);
            }
        }
    }
}

/// Reports the conversion of `expression` into `target` at `location`, which is the call of an operation
/// that a builtin operator call is evaluated as, and the expression itself otherwise
fn validate_typed_conversion<T: AnnotationMap>(
    validator: &mut Validator,
    target: &DataType,
    expression: &AstNode,
    location: &AstNode,
    site: Site,
    context: &ValidationContext<T>,
) {
    let expression = peel(expression);
    let Some(source) = context.annotations.get_type(expression, context.index) else {
        return;
    };

    let is_non_negative_constant = || {
        constant_integer_value(expression, context).is_some_and(|value| value >= 0) || is_size_of(expression)
    };
    let conversion = classify(source, target, context.index).filter(|conversion| match conversion {
        // an operand that changes its sign is reported already, the result follows from it
        Conversion::SignChange => {
            !is_non_negative_constant() && !has_sign_changing_operand(expression, context)
        }
        Conversion::Time => !mixes_date_or_time_operands(expression, context),
        Conversion::Bool => site == Site::Assignment,
        _ => true,
    });

    if let Some(conversion) = conversion {
        let source_name = validator.get_type_name_or_slice(source);
        report(validator, &conversion, site, &source_name, target, peel(location));
    }
}

fn validate_literal_conversion<T: AnnotationMap>(
    validator: &mut Validator,
    target: &DataType,
    value: i128,
    literal: &AstNode,
    site: Site,
    context: &ValidationContext<T>,
) {
    let target_information =
        context.index.get_intrinsic_type_by_name(target.get_name()).get_type_information();

    if target_information.is_date_or_time_type() && site == Site::Assignment {
        report(validator, &Conversion::Time, site, &literal_type_name(value), target, literal);
    } else {
        validate_literal_sign(validator, target, value, literal, site, context);
    }
}

fn validate_literal_sign<T: AnnotationMap>(
    validator: &mut Validator,
    target: &DataType,
    value: i128,
    literal: &AstNode,
    site: Site,
    context: &ValidationContext<T>,
) {
    let target_information =
        context.index.get_intrinsic_type_by_name(target.get_name()).get_type_information();
    if value < 0 && is_unsigned_integer(target_information) {
        report(validator, &Conversion::SignChange, site, &literal_type_name(value), target, literal);
    }
}

fn report(
    validator: &mut Validator,
    conversion: &Conversion,
    site: Site,
    source: &str,
    target: &DataType,
    location: &AstNode,
) {
    let target = validator.get_type_name_or_slice(target);
    validator.push_diagnostic(
        Diagnostic::new(conversion.message(site, source, &target))
            .with_error_code(conversion.code(site))
            .with_location(location),
    );
}

fn classify(source: &DataType, target: &DataType, index: &Index) -> Option<Conversion> {
    // the value of an enum or a subrange has a range of its own
    let is_restricted = index.find_effective_type_by_name(source.get_name()).is_some_and(|ty| {
        matches!(
            ty.get_type_information(),
            DataTypeInformation::Enum { .. } | DataTypeInformation::SubRange { .. }
        )
    });
    if is_restricted {
        return None;
    }

    let source = index.get_intrinsic_type_by_name(source.get_name());
    let target = index.get_intrinsic_type_by_name(target.get_name());
    if source.get_name() == target.get_name() {
        return None;
    }

    let (source, target) = (source.get_type_information(), target.get_type_information());
    if !is_scalar(source) || !is_scalar(target) {
        return None;
    }

    if source.is_date_or_time_type() || target.is_date_or_time_type() {
        return Some(Conversion::Time);
    }

    // BOOL to a number is an invalid assignment already
    if source.is_bool() {
        return None;
    }

    if target.is_bool() {
        return Some(Conversion::Bool);
    }

    match (source, target) {
        (
            DataTypeInformation::Integer { signed: source_signed, size: source_size, .. },
            DataTypeInformation::Integer { signed: target_signed, size: target_size, .. },
        ) => {
            let holds_every_value = !source_signed && *target_signed && target_size > source_size;
            (source_signed != target_signed && !holds_every_value).then_some(Conversion::SignChange)
        }
        (DataTypeInformation::Integer { size, .. }, DataTypeInformation::Float { size: float_size, .. }) => {
            let mantissa_bits = if *float_size == 32 { 24 } else { 53 };
            (*size > mantissa_bits).then_some(Conversion::IntToFloat)
        }
        (DataTypeInformation::Float { .. }, DataTypeInformation::Integer { .. }) => {
            Some(Conversion::FloatToInt)
        }
        _ => None,
    }
}

fn is_scalar(information: &DataTypeInformation) -> bool {
    matches!(information, DataTypeInformation::Integer { .. } | DataTypeInformation::Float { .. })
        && !information.is_character()
}

fn is_unsigned_integer(information: &DataTypeInformation) -> bool {
    information.is_unsigned_int() && !information.is_bool() && !information.is_character()
}

fn peel(node: &AstNode) -> &AstNode {
    match node.get_stmt() {
        AstStatement::ParenExpression(inner) => peel(inner),
        _ => node,
    }
}

/// Literals, with or without a type, are typed as a literal by the resolver, so the operation takes the type
/// of the other operand
fn is_literal_operand(node: &AstNode) -> bool {
    match node.get_stmt_peeled() {
        AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Cast(value), .. }) => {
            matches!(value.get_stmt_peeled(), AstStatement::Literal(_))
        }
        _ => untyped_integer_literal(node).is_some(),
    }
}

/// The value of a constant variable that is initialized with an integer literal
fn constant_integer_value<T: AnnotationMap>(node: &AstNode, context: &ValidationContext<T>) -> Option<i128> {
    let path = node.get_flat_reference_name()?;
    let variable = context.index.find_variable(context.qualifier, &[path])?;
    if !variable.is_constant() {
        return None;
    }

    context
        .index
        .get_const_expressions()
        .maybe_get_constant_statement(&variable.initial_value)
        .and_then(AstNode::get_literal_integer_value)
}

/// The value of an integer literal without a type, such as `5`, `-5` or `NOT 5`
fn untyped_integer_literal(node: &AstNode) -> Option<i128> {
    match node.get_stmt_peeled() {
        AstStatement::Literal(AstLiteral::Integer(value)) => Some(*value),
        AstStatement::UnaryExpression(UnaryExpression { operator: Operator::Minus, value }) => {
            untyped_integer_literal(value).map(|value| -value)
        }
        AstStatement::UnaryExpression(UnaryExpression { operator: Operator::Not, value }) => {
            untyped_integer_literal(value).map(|value| !value)
        }
        _ => None,
    }
}

/// The smallest type that holds the value of an untyped integer literal, signed unless only an unsigned type
/// holds it
fn literal_type_name(value: i128) -> String {
    let literal = AstLiteral::Integer(value);
    match get_literal_actual_signed_type_name(&literal, true) {
        Some(VOID_TYPE) | None => get_literal_actual_signed_type_name(&literal, false).unwrap_or(VOID_TYPE),
        Some(name) => name,
    }
    .to_string()
}

/// Whether an operand of the binary expression is converted to a type of the other sign
fn has_sign_changing_operand<T: AnnotationMap>(node: &AstNode, context: &ValidationContext<T>) -> bool {
    let AstStatement::BinaryExpression(BinaryExpression { left, right, .. }) = node.get_stmt_peeled() else {
        return false;
    };

    [left, right].into_iter().any(|operand| {
        let (Some(source), Some(target)) = (
            context.annotations.get_type(operand, context.index),
            context.annotations.get_type_hint(operand, context.index),
        ) else {
            return false;
        };
        untyped_integer_literal(operand).is_none()
            && matches!(classify(source, target, context.index), Some(Conversion::SignChange))
    })
}

/// Whether a time or date operand of the binary expression is used with an operand of another type
fn mixes_date_or_time_operands<T: AnnotationMap>(node: &AstNode, context: &ValidationContext<T>) -> bool {
    let AstStatement::BinaryExpression(BinaryExpression { left, right, .. }) = node.get_stmt_peeled() else {
        return false;
    };

    let intrinsic_name = |node: &AstNode| {
        context
            .annotations
            .get_type(node, context.index)
            .map(|ty| context.index.get_intrinsic_type_by_name(ty.get_name()).get_name())
    };
    (is_date_or_time(left, context) || is_date_or_time(right, context))
        && intrinsic_name(left) != intrinsic_name(right)
}

fn is_date_or_time<T: AnnotationMap>(node: &AstNode, context: &ValidationContext<T>) -> bool {
    context.annotations.get_type(node, context.index).is_some_and(|ty| {
        context.index.get_intrinsic_type_by_name(ty.get_name()).get_type_information().is_date_or_time_type()
    })
}

fn is_pointer<T: AnnotationMap>(node: &AstNode, context: &ValidationContext<T>) -> bool {
    context
        .annotations
        .get_type(node, context.index)
        .is_some_and(|ty| context.index.get_effective_type_or_void_by_name(ty.get_name()).is_pointer())
}

/// The size of a type is never negative
fn is_size_of(node: &AstNode) -> bool {
    matches!(
        node.get_stmt_peeled(),
        AstStatement::CallStatement(CallStatement { operator, .. })
            if operator.get_flat_reference_name() == Some("SIZEOF")
    )
}
