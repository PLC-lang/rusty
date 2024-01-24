use std::{collections::HashSet, mem::discriminant};

use plc_ast::{
    ast::{
        flatten_expression_list, AstNode, AstStatement, DirectAccess, DirectAccessType, JumpStatement,
        Operator, ReferenceAccess,
    },
    control_statements::{AstControlStatement, ConditionalBlock},
    literals::{Array, AstLiteral, StringValue},
};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;

use super::{array::validate_array_assignment, ValidationContext, Validator, Validators};
use crate::validation::statement::helper::get_datatype_name_or_slice;
use crate::{
    builtins::{self, BuiltIn},
    codegen::generators::expression_generator::get_implicit_call_parameter,
    index::{ArgumentType, Index, PouIndexEntry, VariableIndexEntry, VariableType},
    resolver::{const_evaluator, AnnotationMap, StatementAnnotation},
    typesystem::{
        self, get_equals_function_name_for, get_literal_actual_signed_type_name, DataType,
        DataTypeInformation, Dimension, StructSource, BOOL_TYPE, POINTER_SIZE,
    },
};

macro_rules! visit_all_statements {
    ($validator:expr, $context:expr, $last:expr ) => {
        visit_statement($validator, $last, $context);
    };

    ($validator:expr, $context:expr, $head:expr, $($tail:expr), +) => {
      visit_statement($validator, $head, $context);
      visit_all_statements!($validator, $context, $($tail),+)
    };
}

pub fn visit_statement<T: AnnotationMap>(
    validator: &mut Validator,
    statement: &AstNode,
    context: &ValidationContext<T>,
) {
    match statement.get_stmt() {
        // AstStatement::EmptyStatement { location, id } => (),
        // AstStatement::DefaultValue { location, id } => (),
        // AstStatement::LiteralInteger { value, location, id } => (),
        // AstStatement::LiteralDate { year, month, day, location, id } => (),
        // AstStatement::LiteralDateAndTime { year, month, day, hour, min, sec, nano, location, id } => (),
        // AstStatement::LiteralTimeOfDay { hour, min, sec, nano, location, id } => (),
        // AstStatement::LiteralTime { day, hour, min, sec, milli, micro, nano, negative, location, id } => (),
        // AstStatement::LiteralReal { value, location, id } => (),
        // AstStatement::LiteralBool { value, location, id } => (),
        // AstStatement::LiteralString { value, is_wide, location, id } => (),
        AstStatement::Literal(AstLiteral::Array(Array { elements: Some(elements) })) => {
            visit_statement(validator, elements.as_ref(), context);
        }
        AstStatement::CastStatement(data) => {
            if let AstStatement::Literal(literal) = data.target.get_stmt() {
                validate_cast_literal(
                    validator,
                    literal,
                    statement,
                    &data.type_name,
                    &statement.get_location(),
                    context,
                );
            }
        }
        AstStatement::MultipliedStatement(data) => {
            visit_statement(validator, &data.element, context);
        }
        AstStatement::ReferenceExpr(data) => {
            if let Some(base) = &data.base {
                visit_statement(validator, base, context);
            }

            validate_reference_expression(&data.access, validator, context, statement, &data.base);
        }
        AstStatement::BinaryExpression(data) => {
            visit_all_statements!(validator, context, &data.left, &data.right);
            visit_binary_expression(validator, statement, &data.operator, &data.left, &data.right, context);
        }
        AstStatement::UnaryExpression(data) => {
            visit_statement(validator, &data.value, context);
        }
        AstStatement::ExpressionList(expressions) => {
            expressions.iter().for_each(|element| visit_statement(validator, element, context))
        }
        AstStatement::RangeStatement(data) => {
            visit_all_statements!(validator, context, &data.start, &data.end);
        }
        AstStatement::Assignment(data) => {
            visit_statement(validator, &data.left, context);
            visit_statement(validator, &data.right, context);

            validate_assignment(validator, &data.right, Some(&data.left), &statement.get_location(), context);
            validate_array_assignment(validator, context, statement);
        }
        AstStatement::OutputAssignment(data) => {
            visit_statement(validator, &data.left, context);
            visit_statement(validator, &data.right, context);

            validate_assignment(validator, &data.right, Some(&data.left), &statement.get_location(), context);
        }
        AstStatement::CallStatement(data) => {
            validate_call(validator, &data.operator, data.parameters.as_deref(), &context.set_is_call());
        }
        AstStatement::ControlStatement(kind) => validate_control_statement(validator, kind, context),
        AstStatement::CaseCondition(condition) => {
            // if we get here, then a `CaseCondition` is used outside a `CaseStatement`
            // `CaseCondition` are used as a marker for `CaseStatements` and are not passed as such to the `CaseStatement.case_blocks`
            // see `control_parser` `parse_case_statement()`
            validator.push_diagnostic(
                Diagnostic::error("Case condition used outside of case statement! Did you mean to use ';'?")
                    .with_error_code("E079")
                    .with_location(condition.get_location()),
            );
            visit_statement(validator, condition, context);
        }
        AstStatement::JumpStatement(JumpStatement { condition, target }) => {
            visit_statement(validator, condition, context);
            if context.annotations.get(statement).is_none() {
                validator.push_diagnostic(Diagnostic::unresolved_reference(
                    target.get_flat_reference_name().unwrap_or_default(),
                    statement.get_location(),
                ))
            }
        }
        // AstStatement::ExitStatement { location, id } => (),
        // AstStatement::ContinueStatement { location, id } => (),
        // AstStatement::ReturnStatement { location, id } => (),
        // AstStatement::LiteralNull { location, id } => (),
        AstStatement::ParenExpression(expr) => visit_statement(validator, expr, context),
        _ => {}
    }
    validate_type_nature(validator, statement, context);
}

fn validate_reference_expression<T: AnnotationMap>(
    access: &ReferenceAccess,
    validator: &mut Validator,
    context: &ValidationContext<T>,
    statement: &AstNode,
    base: &Option<Box<AstNode>>,
) {
    match access {
        ReferenceAccess::Member(m) => {
            visit_statement(validator, m.as_ref(), context);

            if let Some(reference_name) = statement.get_flat_reference_name() {
                validate_reference(
                    validator,
                    statement,
                    base.as_deref(),
                    reference_name,
                    &m.get_location(),
                    context,
                );
            }
            validate_direct_access(m, base.as_deref(), context, validator);
        }
        ReferenceAccess::Index(i) => {
            if let Some(base) = base {
                visit_array_access(validator, base, i, context)
            } else {
                validator.push_diagnostic(
                    Diagnostic::error("Index-Access requires an array-value.")
                        .with_error_code("E069")
                        .with_location(statement.get_location()),
                );
            }
        }
        ReferenceAccess::Cast(c) => {
            visit_statement(validator, c.as_ref(), context);

            // see if we try to cast a literal
            if let (AstStatement::Literal(literal), Some(StatementAnnotation::Type { type_name })) =
                (c.get_stmt(), base.as_ref().and_then(|it| context.annotations.get(it)))
            {
                validate_cast_literal(
                    validator,
                    literal,
                    c.as_ref(),
                    type_name.as_str(),
                    &statement.get_location(),
                    context,
                );
            }
        }
        ReferenceAccess::Deref => {
            if base.is_none() {
                validator.diagnostics.push(
                    Diagnostic::error("Dereferencing requires a pointer-value.")
                        .with_error_code("E068")
                        .with_location(statement.get_location()),
                );
            }
        }
        ReferenceAccess::Address => {
            if let Some(base) = base {
                validate_address_of_expression(validator, base, statement.get_location(), context);
            } else {
                validator.diagnostics.push(
                    Diagnostic::error("Address-of requires a value.")
                        .with_error_code("E070")
                        .with_location(statement.get_location()),
                );
            }
        }
    }
}

fn validate_address_of_expression<T: AnnotationMap>(
    validator: &mut Validator,
    target: &AstNode,
    location: SourceLocation,
    context: &ValidationContext<T>,
) {
    if let AstStatement::ParenExpression(expr) = &target.stmt {
        validate_address_of_expression(validator, expr, location, context);
        return;
    }

    let a = context.annotations.get(target);

    if !matches!(a, Some(StatementAnnotation::Variable { .. })) && !target.is_array_access() {
        validator.push_diagnostic(
            Diagnostic::error("Invalid address-of operation").with_error_code("E066").with_location(location),
        );
    }
}

fn validate_direct_access<T: AnnotationMap>(
    m: &AstNode,
    base: Option<&AstNode>,
    context: &ValidationContext<T>,
    validator: &mut Validator,
) {
    if let (AstStatement::DirectAccess(DirectAccess { access, index }), Some(base_annotation)) = (
        m.get_stmt(),
        // FIXME: should we consider the hint if one is available?
        base.and_then(|base| context.annotations.get(base)),
    ) {
        let base_type = context
            .annotations
            .get_type_for_annotation(context.index, base_annotation)
            .unwrap_or(context.index.get_void_type())
            .get_type_information();
        if base_type.is_int() && helper::is_compatible(access, base_type, context.index) {
            validate_access_index(validator, context, index, access, base_type, &m.get_location());
        } else {
            validator.push_diagnostic(
                Diagnostic::error(format!(
                    "{access:?}-Wise access requires a Numerical type larger than {} bits",
                    access.get_bit_width()
                ))
                .with_error_code("E055")
                .with_location(m.get_location()),
            )
        }
    }
}

fn validate_control_statement<T: AnnotationMap>(
    validator: &mut Validator,
    control_statement: &AstControlStatement,
    context: &ValidationContext<T>,
) {
    match control_statement {
        AstControlStatement::If(stmt) => {
            stmt.blocks.iter().for_each(|b| {
                visit_statement(validator, b.condition.as_ref(), context);
                b.body.iter().for_each(|s| visit_statement(validator, s, context));
            });
            stmt.else_block.iter().for_each(|e| visit_statement(validator, e, context));
        }
        AstControlStatement::ForLoop(stmt) => {
            visit_all_statements!(validator, context, &stmt.counter, &stmt.start, &stmt.end);
            if let Some(by_step) = &stmt.by_step {
                visit_statement(validator, by_step, context);
            }
            stmt.body.iter().for_each(|s| visit_statement(validator, s, context));
        }
        AstControlStatement::WhileLoop(stmt) | AstControlStatement::RepeatLoop(stmt) => {
            visit_statement(validator, &stmt.condition, context);
            stmt.body.iter().for_each(|s| visit_statement(validator, s, context));
        }
        AstControlStatement::Case(stmt) => {
            validate_case_statement(validator, &stmt.selector, &stmt.case_blocks, &stmt.else_block, context);
        }
    }
}

/// validates a literal statement with a dedicated type-prefix (e.g. INT#3)
/// checks whether the type-prefix is valid and if the target is a literal
fn validate_cast_literal<T: AnnotationMap>(
    // TODO: i feel like literal is misleading here. can be a reference aswell (INT#x)
    validator: &mut Validator,
    literal: &AstLiteral,
    statement: &AstNode,
    type_name: &str,
    location: &SourceLocation,
    context: &ValidationContext<T>,
) {
    fn incompatible_literal_cast(
        cast_type: &str,
        literal_type: &str,
        location: SourceLocation,
    ) -> Diagnostic {
        Diagnostic::error(format!("Literal {literal_type} is not compatible to {cast_type}"))
            .with_error_code("E054")
            .with_location(location)
    }

    fn literal_out_of_range(literal: &str, range_hint: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::error(format!("Literal {literal} out of range ({range_hint})"))
            .with_error_code("E053")
            .with_location(location)
    }

    let cast_type = context.index.get_effective_type_or_void_by_name(type_name).get_type_information();
    let literal_type = context.index.get_type_information_or_void(
        get_literal_actual_signed_type_name(literal, !cast_type.is_unsigned_int())
            .or_else(|| context.annotations.get_type_hint(statement, context.index).map(DataType::get_name))
            .unwrap_or_else(|| context.annotations.get_type_or_void(statement, context.index).get_name()),
    );

    if !literal.is_cast_prefix_eligible() {
        validator.push_diagnostic(
            Diagnostic::error(format!(
                "Cannot cast into {}, only elementary types are allowed",
                validator.context.slice(&statement.get_location())
            ))
            .with_error_code("E061")
            .with_location(location.clone()),
        )
    } else if cast_type.is_date_or_time_type() || literal_type.is_date_or_time_type() {
        validator.push_diagnostic(incompatible_literal_cast(
            cast_type.get_name(),
            literal_type.get_name(),
            location.clone(),
        ));
        // see if target and cast_type are compatible
    } else if cast_type.is_int() && literal_type.is_int() {
        // INTs with INTs
        if cast_type.get_semantic_size(context.index) < literal_type.get_semantic_size(context.index) {
            validator.push_diagnostic(literal_out_of_range(
                literal.get_literal_value().as_str(),
                cast_type.get_name(),
                location.clone(),
            ));
        }
    } else if cast_type.is_character() && literal_type.is_string() {
        let value = literal.get_literal_value();
        // value contains "" / ''
        if value.len() > 3 {
            validator.push_diagnostic(literal_out_of_range(
                value.as_str(),
                cast_type.get_name(),
                location.clone(),
            ));
        }
    } else if discriminant(cast_type) != discriminant(literal_type) {
        // different types
        // REAL#100 is fine, other differences are not
        if !(cast_type.is_float() && literal_type.is_int()) {
            validator.push_diagnostic(incompatible_literal_cast(
                cast_type.get_name(),
                literal.get_literal_value().as_str(),
                location.clone(),
            ));
        }
    }
}

fn validate_access_index<T: AnnotationMap>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    access_index: &AstNode,
    access_type: &DirectAccessType,
    target_type: &DataTypeInformation,
    location: &SourceLocation,
) {
    match *access_index.get_stmt() {
        AstStatement::Literal(AstLiteral::Integer(value)) => {
            if !helper::is_in_range(
                access_type,
                value.try_into().unwrap_or_default(),
                target_type,
                context.index,
            ) {
                let range = helper::get_range(access_type, target_type, context.index);
                validator.push_diagnostic(
                    Diagnostic::error(format!(
                        "{access_type:?}-Wise access for type {} must be in range {}..{}",
                        target_type.get_name(),
                        &range.start,
                        &range.end
                    ))
                    .with_error_code("E057")
                    .with_location(location.clone()),
                )
            }
        }
        AstStatement::ReferenceExpr(_) => {
            let ref_type = context.annotations.get_type_or_void(access_index, context.index);
            if !ref_type.get_type_information().is_int() {
                validator.push_diagnostic(
                    Diagnostic::error(format!("Invalid type {} for direct variable access. Only variables of Integer types are allowed", ref_type.get_name()))
                    .with_error_code("E056")
                    .with_location(location.clone())
                )
            }
        }
        _ => unreachable!(),
    }
}

fn validate_reference<T: AnnotationMap>(
    validator: &mut Validator,
    statement: &AstNode,
    base: Option<&AstNode>,
    ref_name: &str,
    location: &SourceLocation,
    context: &ValidationContext<T>,
) {
    // unresolved reference
    if !context.annotations.has_type_annotation(statement) {
        validator.push_diagnostic(Diagnostic::unresolved_reference(ref_name, location.clone()));

        // was this meant as a direct access?
        // TODO: find a way to solve this without re-resolving this name
        if let Some(alternative_target_type) =
            context.index.find_variable(context.qualifier, &[ref_name]).and_then(|alternative_target| {
                context.index.find_effective_type_by_name(alternative_target.get_type_name())
            })
        {
            if base.is_some() && (alternative_target_type.is_numerical() || alternative_target_type.is_enum())
            {
                // we accessed a member that does not exist, but we could find a global/local variable that fits
                validator.push_diagnostic(
                    Diagnostic::error(format!("If you meant to directly access a bit/byte/word/..., use %X/%B/%W{ref_name} instead."))
                    .with_error_code("E060")
                    .with_location(location.clone())
                );
            }
        }
    } else if let Some(StatementAnnotation::Variable { qualified_name, argument_type, .. }) =
        context.annotations.get(statement)
    {
        // check if we're accessing a private variable AND the variable's qualifier is not the
        // POU we're accessing it from
        if argument_type.is_private()
            && context
                .qualifier
                .and_then(|qualifier| context.index.find_pou(qualifier))
                .map(|pou| (pou.get_name(), pou.get_container())) // get the container pou (for actions this is the program/fb)
                .map_or(false, |(pou, container)| {
                    !qualified_name.starts_with(pou) && !qualified_name.starts_with(container)
                })
        {
            validator.push_diagnostic(
                //TODO: maybe default to warning?
                Diagnostic::error(format!("Illegal access to private member {qualified_name}"))
                    .with_error_code("E049")
                    .with_location(location.clone()),
            );
        }
    }
}

fn visit_array_access<T: AnnotationMap>(
    validator: &mut Validator,
    reference: &AstNode,
    access: &AstNode,
    context: &ValidationContext<T>,
) {
    let target_type = context.annotations.get_type_or_void(reference, context.index).get_type_information();

    match target_type {
        DataTypeInformation::Array { dimensions, .. } => match access.get_stmt() {
            AstStatement::ExpressionList(expressions) => {
                validate_array_access_dimensions(dimensions.len(), expressions.len(), validator, access);

                for (i, exp) in expressions.iter().enumerate() {
                    validate_array_access(validator, exp, dimensions, i, context);
                }
            }

            _ => {
                validate_array_access_dimensions(dimensions.len(), 1, validator, access);
                validate_array_access(validator, access, dimensions, 0, context)
            }
        },

        DataTypeInformation::Struct {
            source: StructSource::Internal(typesystem::InternalType::VariableLengthArray { ndims, .. }),
            ..
        } => {
            let dims = match access.get_stmt() {
                AstStatement::ExpressionList(expressions) => expressions.len(),
                _ => 1,
            };

            validate_array_access_dimensions(*ndims, dims, validator, access);
        }

        _ => validator.push_diagnostic(
            Diagnostic::error(format!(
                "Invalid type {} for array access. Only variables of Array types are allowed",
                target_type.get_name()
            ))
            .with_error_code("E059")
            .with_location(access.get_location()),
        ),
    }
}

fn validate_array_access_dimensions(ndims: usize, dims: usize, validator: &mut Validator, access: &AstNode) {
    if ndims != dims {
        validator.push_diagnostic(
            Diagnostic::error(format!("Expected array access with {ndims} dimensions, found {dims}"))
                .with_error_code("E045")
                .with_location(access.get_location()),
        )
    }
}

fn validate_array_access<T: AnnotationMap>(
    validator: &mut Validator,
    access: &AstNode,
    dimensions: &[Dimension],
    dimension_index: usize,
    context: &ValidationContext<T>,
) {
    if let AstStatement::Literal(AstLiteral::Integer(value)) = access.get_stmt() {
        if let Some(dimension) = dimensions.get(dimension_index) {
            if let Ok(range) = dimension.get_range(context.index) {
                if !(range.start as i128 <= *value && range.end as i128 >= *value) {
                    validator.push_diagnostic(
                        Diagnostic::error(format!(
                            "Array access must be in the range {}..{}",
                            range.start, range.end
                        ))
                        .with_error_code("E058")
                        .with_location(access.get_location()),
                    )
                }
            }
        }
    } else {
        let type_info = context.annotations.get_type_or_void(access, context.index).get_type_information();
        if !type_info.is_int() {
            validator.push_diagnostic(
                    Diagnostic::error(format!(
                            "Invalid type {} for array access. Only variables of Integer types are allowed to access an array",
                            type_info.get_name()
                    ))
                    .with_error_code("E059")
                    .with_location(access.get_location())
            )
        }
    }
}

fn visit_binary_expression<T: AnnotationMap>(
    validator: &mut Validator,
    statement: &AstNode,
    operator: &Operator,
    left: &AstNode,
    right: &AstNode,
    context: &ValidationContext<T>,
) {
    match operator {
        Operator::Equal => {
            if context.annotations.get_type_hint(statement, context.index).is_none() {
                let lhs = validator.context.slice(&left.location);
                let rhs = validator.context.slice(&right.location);

                validator.push_diagnostic(
                    Diagnostic::error(format!(
                        "This equal statement has no effect, did you mean `{lhs} := {rhs}`?"
                    ))
                    .with_error_code("E023")
                    .with_location(statement.get_location()),
                );
            }

            validate_binary_expression(validator, statement, operator, left, right, context)
        }
        Operator::NotEqual => {
            validate_binary_expression(validator, statement, &Operator::Equal, left, right, context)
        }
        Operator::GreaterOrEqual => {
            // check for the > operator
            validate_binary_expression(validator, statement, &Operator::Greater, left, right, context);
            // check for the = operator
            validate_binary_expression(validator, statement, &Operator::Equal, left, right, context);
        }
        Operator::LessOrEqual => {
            // check for the < operator
            validate_binary_expression(validator, statement, &Operator::Less, left, right, context);
            // check for the = operator
            validate_binary_expression(validator, statement, &Operator::Equal, left, right, context);
        }
        _ => validate_binary_expression(validator, statement, operator, left, right, context),
    }
}

fn validate_binary_expression<T: AnnotationMap>(
    validator: &mut Validator,
    statement: &AstNode,
    operator: &Operator,
    left: &AstNode,
    right: &AstNode,
    context: &ValidationContext<T>,
) {
    let left_type = context.annotations.get_type_or_void(left, context.index).get_type_information();
    let right_type = context.annotations.get_type_or_void(right, context.index).get_type_information();

    // if the type is a subrange, check if the intrinsic type is numerical
    let is_numerical = context.index.find_intrinsic_type(left_type).is_numerical();

    if std::mem::discriminant(left_type) == std::mem::discriminant(right_type)
        && !(is_numerical || left_type.is_pointer())
    {
        // see if we have the right compare-function (non-numbers are compared using user-defined callback-functions)
        if operator.is_comparison_operator()
            && !compare_function_exists(left_type.get_name(), operator, context)
        {
            validator.push_diagnostic(
                Diagnostic::error(format!(
                    "Missing compare function 'FUNCTION {} : BOOL VAR_INPUT a,b : {}; END_VAR ...'.",
                    crate::typesystem::get_equals_function_name_for(left_type.get_name(), operator)
                        .unwrap_or_default()
                        .as_str(),
                    left_type.get_name(),
                ))
                .with_error_code("E073")
                .with_location(statement.get_location()),
            );
        }
    }
}

fn compare_function_exists<T: AnnotationMap>(
    type_name: &str,
    operator: &Operator,
    context: &ValidationContext<T>,
) -> bool {
    let implementation = get_equals_function_name_for(type_name, operator)
        .as_ref()
        .and_then(|function_name| context.index.find_pou_implementation(function_name));

    if let Some(implementation) = implementation {
        let members = context.index.get_pou_members(implementation.get_type_name());

        // we expect two input parameters and a return-parameter
        if let [VariableIndexEntry {
            data_type_name: type_name_1,
            argument_type: ArgumentType::ByVal(VariableType::Input),
            ..
        }, VariableIndexEntry {
            data_type_name: type_name_2,
            argument_type: ArgumentType::ByVal(VariableType::Input),
            ..
        }, VariableIndexEntry {
            data_type_name: return_type,
            argument_type: ArgumentType::ByVal(VariableType::Return),
            ..
        }] = members
        {
            let type_name_1 = context
                .index
                .get_effective_type_or_void_by_name(type_name_1)
                .get_type_information()
                .get_name();
            let type_name_2 = context
                .index
                .get_effective_type_or_void_by_name(type_name_2)
                .get_type_information()
                .get_name();

            // both parameters must have the same type and the return type must be BOOL
            if type_name_1 == type_name && type_name_2 == type_name && return_type == BOOL_TYPE {
                return true;
            }
        }
    }

    false
}

/// Validates if an argument can be passed to a function with [`VariableType::Output`] and
/// [`VariableType::InOut`] parameter types by checking if the argument is a reference (e.g. `foo(x)`) or
/// an assignment (e.g. `foo(x := y)`, `foo(x => y)`). If neither is the case a diagnostic is generated.
fn validate_call_by_ref(validator: &mut Validator, param: &VariableIndexEntry, arg: &AstNode) {
    let ty = param.argument_type.get_inner();
    if !matches!(ty, VariableType::Output | VariableType::InOut) {
        return;
    }

    match (arg.can_be_assigned_to(), arg.get_stmt()) {
        (true, _) => (),

        // Output assignments are optional, e.g. `foo(bar => )` is considered valid
        (false, AstStatement::EmptyStatement(_)) if matches!(ty, VariableType::Output) => (),

        (false, AstStatement::Assignment(data) | AstStatement::OutputAssignment(data)) => {
            validate_call_by_ref(validator, param, &data.right);
        }

        _ => validator.push_diagnostic(
            Diagnostic::error(format!(
                "Expected a reference for parameter {} because their type is {}",
                param.get_name(),
                param.get_variable_type()
            ))
            .with_error_code("E031")
            .with_location(arg.get_location()),
        ),
    }
}

fn validate_assignment<T: AnnotationMap>(
    validator: &mut Validator,
    right: &AstNode,
    left: Option<&AstNode>,
    location: &SourceLocation,
    context: &ValidationContext<T>,
) {
    if let Some(left) = left {
        // Check if we are assigning to a...
        if let Some(StatementAnnotation::Variable { constant, qualified_name, argument_type, .. }) =
            context.annotations.get(left)
        {
            // ...constant variable
            if *constant {
                validator.push_diagnostic(
                    Diagnostic::error(format!("Cannot assign to CONSTANT '{qualified_name}'"))
                        .with_error_code("E036")
                        .with_location(left.get_location()),
                );
            } else {
                // ...enum variable where the RHS does not match its variants
                validate_enum_variant_assignment(
                    validator,
                    context.annotations.get_type_or_void(left, context.index).get_type_information(),
                    context.annotations.get_type_or_void(right, context.index).get_type_information(),
                    qualified_name,
                    right.get_location(),
                );
            }

            // ...VAR_INPUT {ref} variable
            if matches!(argument_type, ArgumentType::ByRef(VariableType::Input)) {
                validator.push_diagnostic(
                    Diagnostic::error("VAR_INPUT {ref} variables are mutable and changes to them will also affect the referenced variable. For increased clarity use VAR_IN_OUT instead.")
                    .with_error_code("E042")
                    .with_location(location.to_owned())
                    );
            }
        }

        // ...or if whatever we got is not assignable, output an error
        if !left.can_be_assigned_to() {
            let expression = validator.context.slice(&left.get_location());
            validator.push_diagnostic(
                Diagnostic::error(format!("Expression {expression} is not assignable."))
                    .with_error_code("E050")
                    .with_location(left.get_location()),
            );
        }
    }

    let right_type = context.annotations.get_type(right, context.index);
    let left_type = context.annotations.get_type_hint(right, context.index);
    if let (Some(right_type), Some(left_type)) = (right_type, left_type) {
        // implicit call parameter assignments are annotated to auto_deref pointers for ´ByRef` parameters
        // we need the inner type
        let left_type = if let DataTypeInformation::Pointer { inner_type_name, auto_deref: true, .. } =
            left_type.get_type_information()
        {
            context.index.get_effective_type_or_void_by_name(inner_type_name)
        } else {
            left_type
        };

        // VLA <- ARRAY assignments are valid when the array is passed to a function expecting a VLA, but
        // are no longer allowed inside a POU body
        if left_type.is_vla() && right_type.is_array() && context.is_call() {
            // TODO: This could benefit from a better error message, tracked in
            // https://github.com/PLC-lang/rusty/issues/118
            validate_variable_length_array_assignment(validator, context, location, left_type, right_type);
            return;
        }

        if !(left_type.is_compatible_with_type(right_type)
            && is_valid_assignment(left_type, right_type, right, context.index, location, validator))
        {
            if left_type.is_pointer() && right_type.is_pointer() {
                validator.push_diagnostic(
                    Diagnostic::error(format!(
                        "Pointers {} and {} have different types",
                        get_datatype_name_or_slice(validator.context, left_type),
                        get_datatype_name_or_slice(validator.context, right_type)
                    ))
                    .with_error_code("E090")
                    .with_location(location.clone()),
                );
            } else {
                validator.push_diagnostic(Diagnostic::invalid_assignment(
                    &get_datatype_name_or_slice(validator.context, right_type),
                    &get_datatype_name_or_slice(validator.context, left_type),
                    location.clone(),
                ));
            }
        } else if right.is_literal() {
            // TODO: See https://github.com/PLC-lang/rusty/issues/857
            // validate_assignment_type_sizes(validator, left_type, right_type, location, context)
        }
    }
}

pub(crate) fn validate_enum_variant_assignment(
    validator: &mut Validator,
    left: &DataTypeInformation,
    right: &DataTypeInformation,
    qualified_name: &str,
    location: SourceLocation,
) {
    if left.is_enum() && left.get_name() != right.get_name() {
        validator.push_diagnostic(
            Diagnostic::error(format!("Assigned value is not a variant of {qualified_name}"))
                .with_error_code("E040")
                .with_location(location),
        )
    }
}

fn validate_variable_length_array_assignment<T: AnnotationMap>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    location: &SourceLocation,
    left_type: &DataType,
    right_type: &DataType,
) {
    let left_inner_type = left_type.get_type_information().get_vla_referenced_type().unwrap();
    let right_inner_type = right_type.get_type_information().get_inner_array_type_name().unwrap();

    let left_dt = context.index.get_effective_type_or_void_by_name(left_inner_type);
    let right_dt = context.index.get_effective_type_or_void_by_name(right_inner_type);

    let left_dims = left_type.get_type_information().get_dimensions().unwrap();
    let right_dims = right_type.get_type_information().get_dimensions().unwrap();

    if left_dt != right_dt || left_dims != right_dims {
        validator.push_diagnostic(Diagnostic::invalid_assignment(
            &get_datatype_name_or_slice(validator.context, right_type),
            &get_datatype_name_or_slice(validator.context, left_type),
            location.clone(),
        ));
    }
}

fn is_valid_assignment(
    left_type: &DataType,
    right_type: &DataType,
    right: &AstNode,
    index: &Index,
    location: &SourceLocation,
    validator: &mut Validator,
) -> bool {
    if is_valid_string_to_char_assignment(
        left_type.get_type_information(),
        right_type.get_type_information(),
        right,
        location,
        validator,
    ) {
        // in this case return true and skip any other validation
        // because those would fail
        return true;
    } else if is_invalid_char_assignment(left_type.get_type_information(), right_type.get_type_information())
    // FIXME: See https://github.com/PLC-lang/rusty/issues/857
        | is_invalid_pointer_assignment(left_type.get_type_information(), right_type.get_type_information(), index, location, validator)
        | is_aggregate_to_none_aggregate_assignment(left_type, right_type)
        | is_aggregate_type_missmatch(left_type, right_type, index)
    {
        return false;
    }
    true
}

/// strings with length 1 can be assigned to characters
fn is_valid_string_to_char_assignment(
    left_type: &DataTypeInformation,
    right_type: &DataTypeInformation,
    right: &AstNode,
    location: &SourceLocation,
    validator: &mut Validator,
) -> bool {
    // TODO: casted literals and reference
    if left_type.is_compatible_char_and_string(right_type) {
        if let AstStatement::Literal(AstLiteral::String(StringValue { value, .. })) = right.get_stmt() {
            if value.len() == 1 {
                return true;
            } else {
                validator.push_diagnostic(
                    Diagnostic::error(
                        format!("Value: '{value}' exceeds length for type: {}", left_type.get_name())
                            .as_str(),
                    )
                    .with_error_code("E065")
                    .with_location(location.clone()),
                );
                return false;
            }
        }
    }
    false
}

fn is_invalid_pointer_assignment(
    left_type: &DataTypeInformation,
    right_type: &DataTypeInformation,
    index: &Index,
    location: &SourceLocation,
    validator: &mut Validator,
) -> bool {
    if left_type.is_pointer() & right_type.is_pointer() {
        return !typesystem::is_same_type_class(left_type, right_type, index);
    }
    //check if Datatype can hold a Pointer (u64)
    else if right_type.is_pointer()
        && !left_type.is_pointer()
        && left_type.get_size_in_bits(index) < POINTER_SIZE
    {
        validator.push_diagnostic(
            Diagnostic::error(format!(
                "The type {} {} is too small to hold a Pointer",
                left_type.get_name(),
                left_type.get_size_in_bits(index)
            ))
            .with_error_code("E065")
            .with_location(location.clone()),
        );
        return true;
    }
    //check if size allocated to Pointer is standart pointer size (u64)
    else if left_type.is_pointer()
        && !right_type.is_pointer()
        && right_type.get_size_in_bits(index) < POINTER_SIZE
    {
        validator.push_diagnostic(
            Diagnostic::error(format!(
                "The type {} {} is too small to be stored in a Pointer",
                right_type.get_name(),
                right_type.get_size_in_bits(index)
            ))
            .with_error_code("E065")
            .with_location(location.clone()),
        );
        return true;
    }
    false
}

/// check if we try to assign a CHAR to WCHAR or vice versa
fn is_invalid_char_assignment(left_type: &DataTypeInformation, right_type: &DataTypeInformation) -> bool {
    if (left_type.is_character() & right_type.is_character())
        && (left_type.get_name() != right_type.get_name())
    {
        return true;
    }
    false
}

/// aggregate types can only be assigned to aggregate types
/// special case char := string_with_length_1, handled by `is_valid_string_to_char_assignment()`
fn is_aggregate_to_none_aggregate_assignment(left_type: &DataType, right_type: &DataType) -> bool {
    left_type.is_aggregate_type() ^ right_type.is_aggregate_type()
}

/// if we try to assign an aggregate type to another
/// check if we have the same type
fn is_aggregate_type_missmatch(left_type: &DataType, right_type: &DataType, index: &Index) -> bool {
    left_type.is_aggregate_type() & right_type.is_aggregate_type()
        && !typesystem::is_same_type_class(
            left_type.get_type_information(),
            right_type.get_type_information(),
            index,
        )
}

fn validate_call<T: AnnotationMap>(
    validator: &mut Validator,
    operator: &AstNode,
    parameters: Option<&AstNode>,
    context: &ValidationContext<T>,
) {
    // visit called pou
    visit_statement(validator, operator, context);

    if let Some(pou) = context.find_pou(operator) {
        // additional validation for builtin calls if necessary
        if let Some(validation) = builtins::get_builtin(pou.get_name()).and_then(BuiltIn::get_validation) {
            validation(validator, operator, parameters, context.annotations, context.index)
        }

        let declared_parameters = context.index.get_declared_parameters(pou.get_name());
        let passed_parameters = parameters.map(flatten_expression_list).unwrap_or_default();

        let mut are_implicit_parameters = true;
        let mut variable_location_in_parent = vec![];

        // validate parameters
        for (i, param) in passed_parameters.iter().enumerate() {
            match get_implicit_call_parameter(param, &declared_parameters, i) {
                Ok((parameter_location_in_parent, right, is_implicit)) => {
                    let left = declared_parameters.get(parameter_location_in_parent);
                    if let Some(left) = left {
                        validate_call_by_ref(validator, left, param);
                        // 'parameter location in parent' and 'variable location in parent' are not the same (e.g VAR blocks are not counted as param).
                        // save actual location in parent for InOut validation
                        variable_location_in_parent.push(left.get_location_in_parent());
                    }

                    // explicit call parameter assignments will be handled by
                    // `visit_statement()` via `Assignment` and `OutputAssignment`
                    if is_implicit {
                        validate_assignment(validator, right, None, &param.get_location(), context);
                    }

                    // mixing implicit and explicit parameters is not allowed
                    // allways compare to the first passed parameter
                    if i == 0 {
                        are_implicit_parameters = is_implicit;
                    } else if are_implicit_parameters != is_implicit {
                        validator.push_diagnostic(
                            Diagnostic::error("Cannot mix implicit and explicit call parameters!")
                                .with_error_code("E031")
                                .with_location(param.get_location()),
                        );
                    }
                }
                Err(err) => {
                    validator.push_diagnostic(
                        Diagnostic::error("Invalid call parameters")
                            .with_error_code("E089")
                            .with_location(param.get_location())
                            .with_sub_diagnostic(err),
                    );
                    break;
                }
            }

            visit_statement(validator, param, context);
        }

        // for PROGRAM/FB we need special inout validation
        if let PouIndexEntry::FunctionBlock { .. } | PouIndexEntry::Program { .. } = pou {
            // pou might actually be an action call: in that case,
            // we need to check if it is called within the context of the parent POU
            // (either the body of the parent or another associated action) => we don't need to validate the params
            if is_action_call_in_qualified_context(context, operator) {
                return;
            }
            let declared_in_out_params: Vec<&&VariableIndexEntry> =
                declared_parameters.iter().filter(|p| VariableType::InOut == p.get_variable_type()).collect();

            // if the called pou has declared inouts, we need to make sure that these were passed to the pou call
            if !declared_in_out_params.is_empty() {
                // check if all inouts were passed to the pou call
                declared_in_out_params.into_iter().for_each(|p| {
                    if !variable_location_in_parent.contains(&p.get_location_in_parent()) {
                        validator.push_diagnostic(
                            Diagnostic::error(format!("Missing inout parameter: {}", p.get_name()))
                                .with_error_code("E030")
                                .with_location(operator.get_location()),
                        );
                    }
                });
            }
        }
    } else {
        // POU could not be found, we can still partially validate the passed parameters
        if let Some(s) = parameters.as_ref() {
            visit_statement(validator, s, context);
        }
    }
}

fn is_action_call_in_qualified_context<T: AnnotationMap>(
    context: &ValidationContext<T>,
    operator: &AstNode,
) -> bool {
    let Some(implementation) = context
        .annotations
        .get_call_name(operator)
        .and_then(|it| context.index.find_implementation_by_name(it))
    else {
        return false;
    };

    if !(implementation.get_implementation_type() == &crate::index::ImplementationType::Action) {
        return false;
    };

    context.qualifier.is_some_and(|qualifier| {
        let pou = context.index.find_pou(qualifier);
        // we are in a qualified context for this action call, i.e. in the parent pou or another associated action
        // => dont validate params
        pou.is_some_and(|it| it.get_container() == implementation.get_type_name())
    })
}

// selector, case_blocks, else_block
fn validate_case_statement<T: AnnotationMap>(
    validator: &mut Validator,
    selector: &AstNode,
    case_blocks: &[ConditionalBlock],
    else_block: &[AstNode],
    context: &ValidationContext<T>,
) {
    visit_statement(validator, selector, context);

    let mut cases = HashSet::new();
    case_blocks.iter().for_each(|b| {
        let condition = b.condition.as_ref();

        // invalid case conditions
        if matches!(condition.get_stmt(), AstStatement::Assignment(_) | AstStatement::CallStatement(_)) {
            validator.push_diagnostic(
                Diagnostic::error("Invalid case condition!")
                    .with_error_code("E079")
                    .with_location(condition.get_location()),
            );
        }

        // validate for duplicate conditions
        // first try to evaluate the conditions value
        const_evaluator::evaluate(condition, context.qualifier, context.index)
            .map_err(|err| {
                // value evaluation and validation not possible with non constants
                validator.push_diagnostic(
                    Diagnostic::error(format!(
                        "{}. Non constant variables are not supported in case conditions",
                        err.get_reason()
                    ))
                    .with_error_code("E080")
                    .with_location(condition.get_location()),
                )
            })
            .map(|v| {
                // check for duplicates if we got a value
                if let Some(AstNode { stmt: AstStatement::Literal(AstLiteral::Integer(value)), .. }) = v {
                    if !cases.insert(value) {
                        validator.push_diagnostic(
                            Diagnostic::error(format!(
                                "Duplicate condition value: {value}. Occurred more than once!"
                            ))
                            .with_error_code("E078")
                            .with_location(condition.get_location()),
                        );
                    }
                };
            })
            .ok(); // no need to worry about the result

        visit_statement(validator, condition, context);
        b.body.iter().for_each(|s| visit_statement(validator, s, context));
    });

    else_block.iter().for_each(|s| visit_statement(validator, s, context));
}

/// Validates that the assigned type and type hint are compatible with the nature for this
/// statement
fn validate_type_nature<T: AnnotationMap>(
    validator: &mut Validator,
    statement: &AstNode,
    context: &ValidationContext<T>,
) {
    if let Some(type_hint) = context
        .annotations
        .get_type_hint(statement, context.index)
        .or_else(|| context.annotations.get_type(statement, context.index))
    {
        if let DataTypeInformation::Generic { generic_symbol, nature, .. } = type_hint.get_type_information()
        {
            // we might be validating an identifier of a formal parameter assignment (FOO(x := 0))
            if let AstStatement::Identifier(_) = statement.get_stmt() {
                return;
            }
            validator.push_diagnostic(
                Diagnostic::error(format!("Could not resolve generic type {generic_symbol} with {nature}"))
                    .with_error_code("E064")
                    .with_location(statement.get_location()),
            );
        } else if let Some((actual_type, generic_nature)) = context
            .annotations
            .get_type(statement, context.index)
            .zip(context.annotations.get_generic_nature(statement))
        {
            // check if type_hint and actual_type is compatible
            // should be handled by assignment validation
            if !(actual_type.has_nature(*generic_nature, context.index)
                // INT parameter for REAL is allowed
                | (type_hint.is_real() & actual_type.is_numerical()))
            {
                validator.push_diagnostic(
                    Diagnostic::error(format!(
                        "Invalid type nature for generic argument. {} is no {}",
                        actual_type.get_name(),
                        generic_nature
                    ))
                    .with_error_code("E062")
                    .with_location(statement.get_location()),
                );
            }
        }
    }
}

fn _validate_assignment_type_sizes<T: AnnotationMap>(
    validator: &mut Validator,
    left: &DataType,
    right: &DataType,
    location: &SourceLocation,
    context: &ValidationContext<T>,
) {
    if left.get_type_information().get_size(context.index)
        < right.get_type_information().get_size(context.index)
    {
        validator.push_diagnostic(
            Diagnostic::error(format!(
                "Potential loss of information due to assigning '{}' to variable of type '{}'.",
                left.get_name(),
                right.get_name()
            ))
            .with_error_code("E067")
            .with_location(location.clone()),
        )
    }
}

mod helper {
    use std::ops::Range;

    use plc_ast::ast::DirectAccessType;
    use plc_index::GlobalContext;

    use crate::typesystem::DataType;
    use crate::{index::Index, typesystem::DataTypeInformation};

    /// Returns true if the current index is in the range for the given type
    pub fn is_in_range(
        access: &DirectAccessType,
        access_index: u64,
        data_type: &DataTypeInformation,
        index: &Index,
    ) -> bool {
        (access.get_bit_width() * access_index) < data_type.get_size_in_bits(index) as u64
    }

    /// Returns the range from 0 for the given data type
    pub fn get_range(
        access: &DirectAccessType,
        data_type: &DataTypeInformation,
        index: &Index,
    ) -> Range<u64> {
        0..((data_type.get_size_in_bits(index) as u64 / access.get_bit_width()) - 1)
    }

    /// Returns true if the direct access can be used for the given type
    pub fn is_compatible(access: &DirectAccessType, data_type: &DataTypeInformation, index: &Index) -> bool {
        data_type.get_semantic_size(index) as u64 > access.get_bit_width()
    }

    pub fn get_datatype_name_or_slice(context: &GlobalContext, dt: &DataType) -> String {
        if dt.is_internal() {
            return dt.get_type_information().get_name().to_string();
        }

        context.slice(&dt.location)
    }
}
