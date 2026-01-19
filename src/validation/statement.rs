use rustc_hash::{FxHashMap, FxHashSet};
use std::mem::discriminant;

use plc_ast::ast::Assignment;
use plc_ast::control_statements::ForLoopStatement;
use plc_ast::{
    ast::{
        flatten_expression_list, AstNode, AstStatement, BinaryExpression, CallStatement, DirectAccess,
        DirectAccessType, JumpStatement, Operator, ReferenceAccess, UnaryExpression,
    },
    control_statements::{AstControlStatement, ConditionalBlock},
    literals::{Array, AstLiteral, StringValue},
};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;

use super::{array::validate_array_assignment, ValidationContext, Validator, Validators};
use crate::index::ImplementationType;
use crate::typesystem::VOID_TYPE;
use crate::validation::statement::helper::get_literal_int_or_const_expr_value;
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

            validate_assignment(validator, &data.right, Some(&data.left), &statement.location, context);
            validate_array_assignment(validator, context, statement);
        }
        AstStatement::OutputAssignment(data) => {
            visit_statement(validator, &data.left, context);
            visit_statement(validator, &data.right, context);

            validate_assignment(validator, &data.right, Some(&data.left), &statement.location, context);
        }
        AstStatement::RefAssignment(data) => {
            visit_statement(validator, &data.left, context);
            visit_statement(validator, &data.right, context);

            validate_ref_assignment(context, validator, data, &statement.location);
            validate_alias_assignment(validator, context, statement);
            validate_array_assignment(validator, context, statement);
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
                Diagnostic::new("Case condition used outside of case statement! Did you mean to use ';'?")
                    .with_error_code("E079")
                    .with_location(condition.as_ref()),
            );
            visit_statement(validator, condition, context);
        }
        AstStatement::JumpStatement(JumpStatement { condition, target }) => {
            visit_statement(validator, condition, context);
            if context.annotations.get(statement).is_none() {
                validator.push_diagnostic(Diagnostic::unresolved_reference(
                    target.get_flat_reference_name().unwrap_or_default(),
                    statement,
                ))
            }
        }
        // AstStatement::ExitStatement { location, id } => (),
        // AstStatement::ContinueStatement { location, id } => (),
        // AstStatement::ReturnStatement { location, id } => (),
        // AstStatement::LiteralNull { location, id } => (),
        AstStatement::ParenExpression(expr) => visit_statement(validator, expr, context),
        AstStatement::Super(_) => {
            if context.is_cast {
                validator.push_diagnostic(
                    Diagnostic::new("The `<type>#` operator cannot be used with `SUPER`")
                        .with_location(statement.get_location())
                        .with_error_code("E119"),
                );
            }

            // do I have a parent_class? if not, this is invalid
            if context
                .qualifier
                .and_then(|it| context.index.find_pou(it).and_then(|it| it.get_super_class()))
                .is_none()
            {
                validator.push_diagnostic(Diagnostic::new("Invalid use of `SUPER`. Usage is only allowed within a POU that directly extends another POU.")
                        .with_location(statement.get_location()).with_error_code("E119"));
            }
        }
        AstStatement::This => {
            if !context.qualifier.is_some_and(|it| {
                context
                    .index
                    .find_pou(it)
                    .and_then(|it| match it {
                        PouIndexEntry::FunctionBlock { .. } => Some(it),
                        PouIndexEntry::Method { parent_name, .. }
                        | PouIndexEntry::Action { parent_name, .. } => context.index.find_pou(parent_name),
                        _ => None,
                    })
                    .is_some_and(|it| it.is_function_block())
            }) {
                validator.push_diagnostic(
                    Diagnostic::new(
                        "Invalid use of `THIS`. Usage is only allowed within `FUNCTION_BLOCK` and its `METHOD`s and `ACTION`s.",
                    )
                    .with_error_code("E120")
                    .with_location(statement),
                );
            }
        }
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
        ReferenceAccess::Global(m) => {
            if m.get_initial_base().or(Some(m)).is_some_and(|it| it.is_super() || it.has_super_metadata()) {
                // super cannot be accessed as a global
                validator.push_diagnostic(
                    Diagnostic::new("`SUPER` is not allowed in global-access position.")
                        .with_location(m.get_location())
                        .with_error_code("E119"),
                );
            };

            validate_member_access(validator, context, statement, m, base);
        }
        ReferenceAccess::Member(m) => {
            if let Some(base) = base {
                if m.is_this() {
                    // this cannot be accessed as a member
                    validator.push_diagnostic(
                        Diagnostic::new("`THIS` is not allowed in member-access position.")
                            .with_location(m.get_location())
                            .with_error_code("E120"),
                    );
                    return;
                }
                if m.is_super() || m.has_super_metadata() {
                    // super cannot be accessed as a member
                    validator.push_diagnostic(
                        Diagnostic::new("`SUPER` is not allowed in member-access position.")
                            .with_location(m.get_location())
                            .with_error_code("E119"),
                    );
                } else if (base.is_super() || base.has_super_metadata())
                    && !(base.is_super_deref() || base.has_super_metadata_deref())
                {
                    validator.push_diagnostic(
                        Diagnostic::new("`SUPER` must be dereferenced to access its members.")
                            .with_location(m.get_location())
                            .with_error_code("E119"),
                    );
                }
            }

            validate_member_access(validator, context, statement, m, base);
        }
        ReferenceAccess::Index(i) => {
            if let Some(base) = base {
                visit_array_access(validator, base, i, context)
            } else {
                validator.push_diagnostic(
                    Diagnostic::new("Index-Access requires an array-value.")
                        .with_error_code("E069")
                        .with_location(statement),
                );
            }
        }
        ReferenceAccess::Cast(c) => {
            visit_statement(validator, c.as_ref(), &context.set_cast());

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
            if base.is_none()
                || base.as_ref().is_some_and(|it| {
                    context
                        .annotations
                        .get_type(it.as_ref(), context.index)
                        .is_some_and(|it| !it.is_pointer())
                })
            {
                validator.diagnostics.push(
                    Diagnostic::new("Dereferencing requires a pointer-value.")
                        .with_error_code("E068")
                        .with_location(statement),
                );
            }
        }
        ReferenceAccess::Address => {
            if let Some(base) = base {
                validate_address_of_expression(
                    validator,
                    base.get_node_peeled(),
                    statement.get_location(),
                    context,
                );
            } else {
                validator.diagnostics.push(
                    Diagnostic::new("Address-of requires a value.")
                        .with_error_code("E070")
                        .with_location(statement),
                );
            }
        }
    }
}

fn validate_member_access<T: AnnotationMap>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    statement: &AstNode,
    member: &AstNode,
    base: &Option<Box<AstNode>>,
) {
    visit_statement(validator, member, context);

    if let Some(reference_name) = statement.get_flat_reference_name() {
        validate_reference(
            validator,
            statement,
            base.as_deref(),
            reference_name,
            &member.get_location(),
            context,
        );
    }
    validate_direct_access(member, base.as_deref(), context, validator);
}

fn validate_address_of_expression<T: AnnotationMap>(
    validator: &mut Validator,
    target: &AstNode,
    location: SourceLocation,
    context: &ValidationContext<T>,
) {
    let a = context.annotations.get(target);

    if !matches!(a, Some(StatementAnnotation::Variable { .. })) && !target.is_array_access() {
        validator.push_diagnostic(
            Diagnostic::new("Invalid address-of operation").with_error_code("E066").with_location(location),
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
                Diagnostic::new(format!(
                    "{access:?}-Wise access requires a Numerical type larger than {} bits",
                    access.get_bit_width()
                ))
                .with_error_code("E055")
                .with_location(m),
            )
        }
    }
}

fn validate_condition<T>(validator: &mut Validator, context: &ValidationContext<T>, condition: &AstNode)
where
    T: AnnotationMap,
{
    if let Some(value) = get_literal_int_or_const_expr_value(condition, context) {
        if value == 0 || value == 1 {
            return;
        }
    }

    let kind = context.annotations.get_type_or_void(condition, context.index);

    if !kind.get_type_information().is_bool() {
        let slice = validator.get_type_name_or_slice(kind);
        let message = format!("Expected a boolean, got `{slice}`");
        let location = condition.get_location();

        let diagnostic = if kind.get_type_information().is_int() {
            // We're a bit more lenient with integers, generating a warning instead of an error
            let message = format!("{message}, consider adding an `=` or `<>` operator for better clarity");
            Diagnostic::new(message).with_location(location).with_error_code("E096")
        } else {
            // ...anything else is a hard error
            Diagnostic::new(message).with_location(location).with_error_code("E094")
        };

        validator.push_diagnostic(diagnostic)
    }
}

fn validate_control_statement<T: AnnotationMap>(
    validator: &mut Validator,
    control_statement: &AstControlStatement,
    context: &ValidationContext<T>,
) {
    match control_statement {
        AstControlStatement::If(stmt) => {
            for block in &stmt.blocks {
                validate_condition(validator, context, &block.condition);
                block.body.iter().for_each(|s| visit_statement(validator, s, context));
            }

            stmt.else_block.iter().for_each(|e| visit_statement(validator, e, context));
        }
        AstControlStatement::ForLoop(stmt) => {
            validate_for_loop(validator, context, stmt);
            visit_all_statements!(validator, context, &stmt.counter, &stmt.start, &stmt.end);
            if let Some(by_step) = &stmt.by_step {
                visit_statement(validator, by_step, context);
            }
            stmt.body.iter().for_each(|s| visit_statement(validator, s, context));
        }
        AstControlStatement::WhileLoop(stmt) | AstControlStatement::RepeatLoop(stmt) => {
            validate_condition(validator, context, &stmt.condition);
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
        Diagnostic::new(format!("Literal {literal_type} is not compatible to {cast_type}"))
            .with_error_code("E054")
            .with_location(location)
    }

    fn literal_out_of_range(literal: &str, range_hint: &str, location: SourceLocation) -> Diagnostic {
        Diagnostic::new(format!("Literal {literal} out of range ({range_hint})"))
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
            Diagnostic::new(format!(
                "Cannot cast into {}, only elementary types are allowed",
                validator.context.slice(&statement.get_location())
            ))
            .with_error_code("E061")
            .with_location(location),
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
                    Diagnostic::new(format!(
                        "{access_type:?}-Wise access for type {} must be in range {}..{}",
                        target_type.get_name(),
                        &range.start,
                        &range.end
                    ))
                    .with_error_code("E057")
                    .with_location(location),
                )
            }
        }
        AstStatement::ReferenceExpr(_) => {
            let Some(ref_type) = context.annotations.get_type(access_index, context.index) else { return };
            if !ref_type.get_type_information().is_int() {
                validator.push_diagnostic(
                    Diagnostic::new(format!("Invalid type {} for direct variable access. Only variables of Integer types are allowed", ref_type.get_name()))
                    .with_error_code("E056")
                    .with_location(location)
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
    if location.is_internal() {
        return;
    }

    // unresolved reference
    if !context.annotations.has_type_annotation(statement) {
        if base.is_some_and(|it| it.has_super_metadata() || it.is_super()) {
            // We don't want to show unresolved reference/bitaccess diagnostics for invalid super accesses without deref
            return;
        }
        // XXX: Temporary solution, is there a better way? Technically we could introduce a diagnostic when
        //      lowering the references to calls, then checking with the index if the defined POU exists but
        //      then we'd get two similar error, one describing what the exact issue is (i.e. no get/set) and
        //      the other describing that it cant find a reference to "__{get,set}_<property name>"
        match ref_name {
            _ if ref_name.starts_with("__set") => {
                validator.push_diagnostic(
                    Diagnostic::new("SET property not defined")
                        .with_error_code("E048")
                        .with_location(location),
                );
                return;
            }

            _ if ref_name.starts_with("__get") => {
                validator.push_diagnostic(
                    Diagnostic::new("GET property not defined")
                        .with_error_code("E048")
                        .with_location(location),
                );
                return;
            }

            _ => (),
        };
        validator.push_diagnostic(Diagnostic::unresolved_reference(ref_name, location));

        // was this meant as a direct access?
        // TODO: find a way to solve this without re-resolving this name
        if let Some(alternative_target_type) =
            context.index.find_variable(context.qualifier, &[ref_name]).and_then(|alternative_target| {
                context.index.find_effective_type_by_name(alternative_target.get_type_name())
            })
        {
            if alternative_target_type.is_numerical() || alternative_target_type.is_enum() {
                // we accessed a member that does not exist, but we could find a global/local variable that fits
                validator.push_diagnostic(
                    Diagnostic::new(format!("If you meant to directly access a bit/byte/word/..., use %X/%B/%W{ref_name} instead."))
                    .with_error_code("E060")
                    .with_location(location)
                );
            }
        }

        return;
    }

    match context.annotations.get(statement) {
        Some(StatementAnnotation::Variable { qualified_name, argument_type, .. }) => {
            // check if we're accessing a private variable AND the variable's qualifier is not the
            // POU we're accessing it from.
            if argument_type.is_private()
                && context
                    .qualifier
                    .and_then(|qualifier| context.index.find_pou(qualifier))
                    .map(|pou| (pou.get_name(), pou.get_container()))
                    .is_some_and(|(pou, container)| {
                        !(qualified_name.starts_with(pou)
                                || qualified_name.starts_with(container)
                                || context.index.is_init_function(pou)
                                //Hack: Avoid internal check here because of the super call
                                || location.is_internal())
                    })
            {
                validator.push_diagnostic(
                    Diagnostic::new(format!("Illegal access to private member {qualified_name}"))
                        .with_error_code("E049")
                        .with_location(location),
                );
            }
        }
        Some(StatementAnnotation::Program { qualified_name }) => {
            if !context.is_call()
                && context
                    .index
                    .find_implementation_by_name(qualified_name)
                    .is_some_and(|it| matches!(it.get_implementation_type(), ImplementationType::Action))
            {
                // we parsed a reference expression to an action but we are not in a call-context: likely an action call without parentheses
                validator.push_diagnostic(
                    Diagnostic::new(format!("A reference to {qualified_name} exists, but it is an ACTION. If you meant to call it, add `()` to the statement: `{qualified_name}()`"))
                        .with_error_code("E095")
                        .with_location(location)
                );
            }
        }
        _ => (),
    }
}

fn visit_array_access<T: AnnotationMap>(
    validator: &mut Validator,
    reference: &AstNode,
    access: &AstNode,
    context: &ValidationContext<T>,
) {
    let Some(target_type) =
        context.annotations.get_type(reference, context.index).map(|it| it.get_type_information())
    else {
        return;
    };

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
            Diagnostic::new(format!(
                "Invalid type {} for array access. Only variables of Array types are allowed",
                target_type.get_name()
            ))
            .with_error_code("E059")
            .with_location(access),
        ),
    }
}

fn validate_array_access_dimensions(ndims: usize, dims: usize, validator: &mut Validator, access: &AstNode) {
    if ndims != dims {
        validator.push_diagnostic(
            Diagnostic::new(format!("Expected array access with {ndims} dimensions, found {dims}"))
                .with_error_code("E045")
                .with_location(access),
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
    visit_statement(validator, access, context);
    if let AstStatement::Literal(AstLiteral::Integer(value)) = access.get_stmt() {
        if let Some(dimension) = dimensions.get(dimension_index) {
            if let Ok(range) = dimension.get_range(context.index) {
                if !(range.start as i128 <= *value && range.end as i128 >= *value) {
                    validator.push_diagnostic(
                        Diagnostic::new(format!(
                            "Array access must be in the range {}..{}",
                            range.start, range.end
                        ))
                        .with_error_code("E058")
                        .with_location(access),
                    )
                }
            }
        }
    } else {
        let Some(type_info) =
            context.annotations.get_type(access, context.index).map(|it| it.get_type_information())
        else {
            return;
        };
        if !type_info.is_int() {
            validator.push_diagnostic(
                    Diagnostic::new(format!(
                            "Invalid type {} for array access. Only variables of Integer types are allowed to access an array",
                            type_info.get_name()
                    ))
                    .with_error_code("E059")
                    .with_location(access)
            )
        }
    }
}

pub fn validate_type_compatibility(
    validator: &mut Validator,
    annotations: &dyn AnnotationMap,
    index: &Index,
    left: &AstNode,
    right: &AstNode,
) {
    let ty_left = annotations.get_type_or_void(left, index);
    let ty_right = annotations.get_type_or_void(right, index);

    validate_type_compatibility_with_data_types(
        validator,
        ty_left,
        ty_right,
        &left.location.span(&right.location),
    );
}

pub fn validate_type_compatibility_with_data_types(
    validator: &mut Validator,
    ty_left: &DataType,
    ty_right: &DataType,
    location: &SourceLocation,
) {
    if !(ty_left.is_compatible_with_type(ty_right) && ty_right.is_compatible_with_type(ty_left)) {
        let ty_left_name = validator.get_type_name_or_slice(ty_left);
        let ty_right_name = validator.get_type_name_or_slice(ty_right);

        validator.push_diagnostic(
            Diagnostic::new(format!(
                "Invalid expression, types {ty_left_name} and {ty_right_name} are incompatible in the given context"
            ))
            .with_error_code("E031")
            .with_location(location),
        );
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
                    Diagnostic::new(format!(
                        "This equal statement has no effect, did you mean `{lhs} := {rhs}`?"
                    ))
                    .with_error_code("E023")
                    .with_location(statement),
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

    validate_type_compatibility(validator, context.annotations, context.index, left, right);
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
    let is_numerical = context.index.get_intrinsic_type_information(left_type).is_numerical();

    if discriminant(left_type) == discriminant(right_type) && !(is_numerical || left_type.is_pointer()) {
        // see if we have the right compare-function (non-numbers are compared using user-defined callback-functions)
        if operator.is_comparison_operator()
            && !compare_function_exists(left_type.get_name(), operator, context)
        {
            validator.push_diagnostic(
                Diagnostic::new(format!(
                    "Missing compare function 'FUNCTION {} : BOOL VAR_INPUT a,b : {}; END_VAR ...'.",
                    get_equals_function_name_for(left_type.get_name(), operator).unwrap_or_default().as_str(),
                    left_type.get_name(),
                ))
                .with_error_code("E073")
                .with_location(statement),
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
            Diagnostic::new(format!(
                "Expected a reference for parameter {} because their type is {}",
                param.get_name(),
                param.get_variable_type()
            ))
            .with_error_code("E031")
            .with_location(arg),
        ),
    }
}

pub fn validate_assignment_mismatch<T>(
    context: &ValidationContext<T>,
    validator: &mut Validator,
    type_lhs: &DataType,
    type_rhs: &DataType,
    assignment_location: &SourceLocation,
) where
    T: AnnotationMap,
{
    let type_info_lhs = context.index.get_intrinsic_type_information(
        context.index.find_elementary_pointer_type(type_lhs.get_type_information()),
    );
    let type_info_rhs = context.index.get_intrinsic_type_information(
        context.index.find_elementary_pointer_type(type_rhs.get_type_information()),
    );

    // We might be dealing with an `ADR` or `REF` call on a `POINTER TO` variable
    if (type_lhs.is_pointer() && !type_lhs.is_type_safe_pointer())
        && (type_rhs.is_pointer() || type_info_rhs.is_ptr_sized_int())
    {
        return;
    }

    if type_info_lhs.is_array() && type_info_rhs.is_array() {
        let len_lhs = type_info_lhs.get_array_length(context.index).unwrap_or_default();
        let len_rhs = type_info_rhs.get_array_length(context.index).unwrap_or_default();

        let inner_ty_name_lhs = type_info_lhs.get_inner_array_type_name().unwrap_or(VOID_TYPE);
        let inner_ty_name_rhs = type_info_rhs.get_inner_array_type_name().unwrap_or(VOID_TYPE);
        let inner_ty_lhs = context.index.find_effective_type_by_name(inner_ty_name_lhs);
        let inner_ty_rhs = context.index.find_effective_type_by_name(inner_ty_name_rhs);

        if len_lhs != len_rhs || inner_ty_lhs != inner_ty_rhs {
            validator.push_diagnostic(Diagnostic::invalid_assignment(
                &validator.get_type_name_or_slice(type_rhs),
                &validator.get_type_name_or_slice(type_lhs),
                assignment_location,
            ));
        }
    } else if type_info_lhs != type_info_rhs {
        if is_related_to(context, type_info_lhs.get_name(), type_info_rhs.get_name()) {
            return;
        }

        let type_name_lhs = validator.get_type_name_or_slice(type_lhs);
        let type_name_rhs = validator.get_type_name_or_slice(type_rhs);

        validator.push_diagnostic(Diagnostic::invalid_assignment(
            &type_name_rhs,
            &type_name_lhs,
            assignment_location,
        ));
    }
}

/// Returns true if the right POU is a direct or indirect child of the left POU
fn is_related_to<T>(context: &ValidationContext<T>, pou_name_lhs: &str, pou_name_rhs: &str) -> bool
where
    T: AnnotationMap,
{
    let Some(pou_lhs) = context.index.find_pou(pou_name_lhs) else {
        return false;
    };

    let Some(pou_rhs) = context.index.find_pou(pou_name_rhs) else {
        return false;
    };

    match pou_rhs.get_super_class() {
        Some(parent) => {
            if pou_lhs.get_name() == parent {
                true
            } else {
                is_related_to(context, pou_name_lhs, parent)
            }
        }

        None => false,
    }
}

/// Checks if `REF=` assignments are correct, specifically if the left-hand side is a reference declared
/// as `REFERENCE TO` and the right hand side is a lvalue of the same type that is being referenced.
fn validate_ref_assignment<T: AnnotationMap>(
    context: &ValidationContext<T>,
    validator: &mut Validator,
    assignment: &Assignment,
    assignment_location: &SourceLocation,
) {
    let annotation_lhs = context.annotations.get(&assignment.left);
    let type_lhs = context.annotations.get_type_or_void(&assignment.left, context.index);
    let type_rhs = context.annotations.get_type_or_void(&assignment.right, context.index);

    // Assert that the right-hand side is a reference
    if !(assignment.right.get_node_peeled().is_reference()
        || assignment_location.is_builtin_internal()
        || assignment.right.is_zero())
    {
        validator.push_diagnostic(
            Diagnostic::new("Invalid assignment, expected a reference")
                .with_location(&assignment.right.location)
                .with_error_code("E098"),
        );
    }

    // Assert that the left-hand side is a valid pointer-reference
    if !type_lhs.is_pointer() && !annotation_lhs.is_some_and(|opt| opt.is_auto_deref()) {
        validator.push_diagnostic(
            Diagnostic::new("Invalid assignment, expected a pointer reference")
                .with_location(&assignment.left.location)
                .with_error_code("E098"),
        )
    }

    // If the right side is a reference, validate type mismatches
    if assignment.right.is_reference() {
        validate_assignment_mismatch(context, validator, type_lhs, type_rhs, assignment_location);
    }
}

/// Returns a diagnostic if an alias declared variables address is re-assigned in the POU body.
fn validate_alias_assignment<T: AnnotationMap>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    ref_assignment: &AstNode,
) {
    if let AstStatement::RefAssignment(Assignment { left, .. }) = ref_assignment.get_stmt() {
        if context
            .annotations
            .get(left)
            .is_some_and(|opt| opt.is_alias() && !ref_assignment.location.is_builtin_internal())
        {
            validator.push_diagnostic(
                Diagnostic::new(format!(
                    "{} is an immutable alias variable, can not change the address",
                    validator.context.slice(&left.location)
                ))
                .with_location(&ref_assignment.location)
                .with_error_code("E100"),
            )
        }
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
                    Diagnostic::new(format!("Cannot assign to CONSTANT '{qualified_name}'"))
                        .with_error_code("E036")
                        .with_location(left),
                );
            } else {
                // ...enum variable where the RHS does not match its variants
                validate_enum_variant_assignment(
                    context,
                    validator,
                    qualified_name,
                    context.annotations.get_type_or_void(left, context.index),
                    right,
                );
            }

            // ...VAR_INPUT {ref} variable
            if matches!(argument_type, ArgumentType::ByRef(VariableType::Input)) {
                validator.push_diagnostic(
                    Diagnostic::new("VAR_INPUT {ref} variables are mutable and changes to them will also affect the referenced variable. For increased clarity use VAR_IN_OUT instead.")
                    .with_error_code("E042")
                    .with_location(location)
                    );
            }
        }

        // ...or if whatever we got is not assignable, output an error
        if !left.can_be_assigned_to() {
            let expression = validator.context.slice(&left.get_location());
            validator.push_diagnostic(
                // TODO: would be nice to have a more specific error message. For instance `THIS`
                // might not assignable because its use is only allowed in FBs and their methods.
                // Same goes for `SUPER`.
                Diagnostic::new(format!("Expression {expression} is not assignable."))
                    .with_error_code("E050")
                    .with_location(left),
            );
        }

        if has_return_assignment_in_void_function(context, left) {
            validator.push_diagnostic(
                Diagnostic::new("Function declared as VOID, but trying to assign a return value")
                    .with_location(location)
                    .with_error_code("E093"),
            )
        }
    }

    let right_type = context.annotations.get_type(right, context.index);
    let left_type = context.annotations.get_type_hint(right, context.index);

    if let (Some(right_type), Some(left_type)) = (right_type, left_type) {
        // implicit call parameter assignments are annotated to auto_deref pointers for ´ByRef` parameters
        // we need the inner type
        let left_type = if let DataTypeInformation::Pointer { inner_type_name, auto_deref: Some(_), .. } =
            left_type.get_type_information()
        {
            context.index.get_effective_type_or_void_by_name(inner_type_name)
        } else {
            left_type
        };

        // VLA <- ARRAY assignments are valid when the array is passed to a function expecting a VLA, but
        // are no longer allowed inside a POU body
        if left_type.is_vla() && right_type.is_array() && context.is_call() {
            validate_variable_length_array_assignment(validator, context, location, left_type, right_type);
            return;
        }

        if !(left_type.is_compatible_with_type(right_type)
            && is_valid_assignment(left_type, right_type, right, context.index, location, validator))
        {
            // TODO: #THIS && !left_type.is_this()
            if left_type.is_type_safe_pointer() && right_type.is_pointer() {
                validator.push_diagnostic(
                    Diagnostic::new(format!(
                        "Pointers {} and {} have different types",
                        validator.get_type_name_or_slice(left_type),
                        validator.get_type_name_or_slice(right_type)
                    ))
                    .with_error_code("E090")
                    .with_location(location),
                );
            } else {
                validate_assignment_mismatch(context, validator, left_type, right_type, location);
            }
        } else {
            validate_assignment_type_sizes(validator, left_type, right, context)
        }
    }
}

/// Returns true if an assignment statement exists such that a return value is assigned to a void
/// function. For example the following will return true
/// ```iecst
/// FUNCTION foo
/// foo := 1; // Doesn't make sense, foo is of type VOID
/// END_FUNCTION
/// ```
fn has_return_assignment_in_void_function<T>(context: &ValidationContext<T>, left: &AstNode) -> bool
where
    T: AnnotationMap,
{
    if let Some((var_name, qualifier)) = left.get_flat_reference_name().zip(context.qualifier) {
        let variable = context.index.find_variable(context.qualifier, &[var_name]);
        let pou = context.index.find_pou(qualifier);

        if variable.is_none() && pou.is_some_and(|fun| fun.is_void_function()) {
            return var_name == qualifier;
        }
    }

    false
}

pub(crate) fn validate_enum_variant_assignment<T: AnnotationMap>(
    context: &ValidationContext<T>,
    validator: &mut Validator,
    qualified_name: &str,
    left_dt: &DataType,
    right: &AstNode,
) {
    if !left_dt.is_enum() {
        return;
    }

    let right_dt = context.annotations.get_type_or_void(right, context.index);

    // For it to be a valid enum assignment, the right-hand side must yield a const-expr value
    // (i.e. literal integer or some enum variant) and the left-hand side (which is an enum) must have that
    // const-expr value as a variant (e.g. the const-expr must be 1 or 2 for `Status : (idle := 1, running := 2)`)
    let Some(value_rhs) = get_literal_int_or_const_expr_value(right, context) else {
        // ...however function calls for example are no const-expr hence only report if datatypes also differ
        if left_dt.get_name() != right_dt.get_name() {
            validator.push_diagnostic(
                Diagnostic::new(format!(
                    "Value evaluated at run-time, use an enum variant from `{}`",
                    validator.get_type_name_or_slice(left_dt)
                ))
                .with_location(right)
                .with_secondary_location(&left_dt.location)
                .with_error_code("E091"),
            );
        }

        return;
    };

    let Some(variable) = context.index.find_fully_qualified_variable(qualified_name) else { return };
    let variants = helper::get_enum_variant_values(context.index, variable);

    match variants.iter().find(|(_, value_lhs)| *value_lhs == value_rhs) {
        Some((variant, _)) => {
            if left_dt.get_name() != right_dt.get_name() {
                validator.push_diagnostic(
                    Diagnostic::new(format!(
                        "Replace `{}` with `{}`",
                        validator.context.slice(&right.location),
                        variant.get_name()
                    ))
                    .with_error_code("E092")
                    .with_location(right)
                    .with_secondary_location(&left_dt.location),
                );
            }
        }
        None => {
            validator.push_diagnostic(
                Diagnostic::new(format!(
                    "Non-standard enum value `{}` for `{}`",
                    validator.context.slice(&right.location),
                    validator.get_type_name_or_slice(left_dt)
                ))
                .with_location(right)
                .with_secondary_location(&left_dt.location)
                .with_error_code("E040"),
            );
        }
    };
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

    let left_dims = left_type.get_type_information().get_dimension_count().unwrap();
    let right_dims = right_type.get_type_information().get_dimension_count().unwrap();

    if left_dt != right_dt || left_dims != right_dims {
        validator.push_diagnostic(Diagnostic::invalid_assignment(
            &validator.get_type_name_or_slice(right_type),
            &validator.get_type_name_or_slice(left_type),
            location,
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
    if right_type.is_void() {
        return false;
    }

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
        | is_invalid_pointer_assignment(
            left_type.get_type_information(),
            right_type.get_type_information(),
            index,
            location,
            validator,
        )
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
                    Diagnostic::new(
                        format!("Value: '{value}' exceeds length for type: {}", left_type.get_name())
                            .as_str(),
                    )
                    .with_error_code("E065")
                    .with_location(location),
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
    else if (right_type.is_pointer() && !right_type.is_auto_deref())
        && !left_type.is_pointer()
        && left_type.get_size_in_bits(index).unwrap_or_default() < POINTER_SIZE
    {
        validator.push_diagnostic(
            Diagnostic::new(format!(
                "The type {} {} is too small to hold a Pointer",
                left_type.get_name(),
                left_type.get_size_in_bits(index).unwrap_or_default()
            ))
            .with_error_code("E065")
            .with_location(location),
        );
        return true;
    }
    //check if size allocated to Pointer is standart pointer size (u64)
    else if left_type.is_pointer()
        && !right_type.is_pointer()
        && right_type.get_size_in_bits(index).unwrap_or_default() < POINTER_SIZE
    {
        validator.push_diagnostic(
            Diagnostic::new(format!(
                "The type {} {} is too small to be stored in a Pointer",
                right_type.get_name(),
                right_type.get_size_in_bits(index).unwrap_or_default()
            ))
            .with_error_code("E065")
            .with_location(location),
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
    let lhs = left_type.get_type_information();
    let rhs = right_type.get_type_information();
    if !(left_type.is_aggregate_type() & right_type.is_aggregate_type()) {
        return false;
    }
    if lhs.is_array() {
        let inner_l = index.get_intrinsic_type_information(
            index.get_type_information_or_void(lhs.get_inner_array_type_name().unwrap_or(VOID_TYPE)),
        );
        let inner_r = index.get_intrinsic_type_information(
            index.get_type_information_or_void(rhs.get_inner_array_type_name().unwrap_or(VOID_TYPE)),
        );
        !(inner_l == inner_r && typesystem::is_same_type_class(lhs, rhs, index))
    } else {
        !typesystem::is_same_type_class(lhs, rhs, index)
    }
}

fn validate_call<T: AnnotationMap>(
    validator: &mut Validator,
    fn_ident: &AstNode,
    fn_args: Option<&AstNode>,
    context: &ValidationContext<T>,
) {
    visit_statement(validator, fn_ident, context);

    if let AstStatement::CallStatement(_) = fn_ident.get_stmt() {
        validator.push_diagnostic(
            Diagnostic::new("Properties cannot be called like functions. Remove `()`")
                .with_error_code("E007")
                .with_location(fn_ident),
        );
    }

    // Check if we're dealing with a builtin function and if so call its validation function
    if let Some(validation) = builtins::get_builtin(fn_ident.get_flat_reference_name().unwrap_or_default())
        .and_then(BuiltIn::get_validation)
    {
        validation(validator, fn_ident, fn_args, context.annotations, context.index);
    }

    let Some(pou) = context.find_pou(fn_ident) else {
        // POU could not be found, we can still partially validate the passed parameters
        if let Some(s) = fn_args.as_ref() {
            visit_statement(validator, s, context);
        }
        return;
    };
    let arguments = fn_args.map(flatten_expression_list).unwrap_or_default();
    let parameters = context.index.get_available_parameters(pou.get_name());

    if builtins::get_builtin(pou.get_name()).is_none() {
        validate_argument_count(context, validator, pou, &arguments, &fn_ident.location);
    }

    let mut arguments_are_implicit = true;
    let mut variable_location_in_parent = vec![];

    // validate parameters
    for (i, argument) in arguments.iter().enumerate() {
        match get_implicit_call_parameter(argument, &parameters, i) {
            Ok((parameter_idx, right, is_implicit)) => {
                if i == 0 {
                    arguments_are_implicit = is_implicit;
                }

                if let Some(left) = parameters.get(parameter_idx) {
                    validate_call_by_ref(validator, left, argument);
                    // 'parameter location in parent' and 'variable location in parent' are not the same (e.g VAR blocks are not counted as param).
                    // save actual location in parent for InOut validation
                    variable_location_in_parent.push(left.get_location_in_parent());
                }

                // explicit call parameter assignments will be handled by
                // `visit_statement()` via `Assignment` and `OutputAssignment`
                if is_implicit {
                    validate_assignment(validator, right, None, &argument.get_location(), context);
                }

                // mixing implicit and explicit arguments is not allowed
                // allways compare to the first argument
                if arguments_are_implicit != is_implicit {
                    validator.push_diagnostic(
                        Diagnostic::new("Cannot mix implicit and explicit call parameters!")
                            .with_error_code("E031")
                            .with_location(*argument),
                    );
                }
            }

            Err(err) => {
                validator.push_diagnostic(
                    Diagnostic::new("Invalid call parameters")
                        .with_error_code("E089")
                        .with_location(*argument)
                        .with_sub_diagnostic(err.into()),
                );
                break;
            }
        }

        visit_statement(validator, argument, context);
    }

    // for PROGRAM/FB we need special inout validation
    if pou.is_stateful() || pou.is_method() {
        // pou might actually be an action call: in that case,
        // we need to check if it is called within the context of the parent POU
        // (either the body of the parent or another associated action) => we don't need to validate the params
        if is_action_call_in_qualified_context(context, fn_ident) {
            return;
        }

        let declared_in_out_params: Vec<&VariableIndexEntry> =
            parameters.into_iter().filter(|param| param.is_inout()).collect();

        if !declared_in_out_params.is_empty() {
            // Check if all IN_OUT arguments were passed by cross-checking with the parameters
            declared_in_out_params.into_iter().for_each(|p| {
                if !variable_location_in_parent.contains(&p.get_location_in_parent()) {
                    validator.push_diagnostic(
                        Diagnostic::new(format!("Argument `{}` is missing", p.get_name()))
                            .with_error_code("E030")
                            .with_location(fn_ident),
                    );
                }
            });
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

    let mut cases = FxHashSet::default();
    case_blocks.iter().for_each(|b| {
        let condition = b.condition.as_ref();

        // invalid case conditions
        if matches!(condition.get_stmt(), AstStatement::Assignment(_) | AstStatement::CallStatement(_)) {
            validator.push_diagnostic(
                Diagnostic::new("Invalid case condition!").with_error_code("E079").with_location(condition),
            );
        }

        // validate for duplicate conditions
        // first try to evaluate the conditions value
        const_evaluator::evaluate(condition, context.qualifier, context.index, None)
            .map_err(|err| {
                // value evaluation and validation not possible with non constants
                validator.push_diagnostic(
                    Diagnostic::new(format!(
                        "{}. Non constant variables are not supported in case conditions",
                        err.get_reason()
                    ))
                    .with_error_code("E080")
                    .with_location(condition),
                )
            })
            .map(|v| {
                // check for duplicates if we got a value
                if let Some(AstNode { stmt: AstStatement::Literal(AstLiteral::Integer(value)), .. }) = v {
                    if !cases.insert(value) {
                        validator.push_diagnostic(
                            Diagnostic::new(format!(
                                "Duplicate condition value: {value}. Occurred more than once!"
                            ))
                            .with_error_code("E078")
                            .with_location(condition),
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

fn validate_for_loop<T: AnnotationMap>(
    validator: &mut Validator,
    context: &ValidationContext<T>,
    statement: &ForLoopStatement,
) {
    statement.get_conditionals().iter().for_each(|node| {
        let kind = context.annotations.get_type_or_void(node, context.index);

        if kind.is_real() || !kind.is_numerical() {
            let slice = validator.get_type_name_or_slice(kind);
            let message = format!("Expected an integer value, got `{slice}`");
            validator.push_diagnostic(Diagnostic::new(message).with_location(*node).with_error_code("E094"));
        }
    })

    // TODO: Check if start, end, counter and the step values have the same type, e.g. all of them have to be DINT
    // TODO: Check if the body doesn't modify the conditional values
    //       NOTE: This requires some analysis feature which we currently lack.
    //       While it might be possible to check if the left-hand side of an assignment is a
    //       conditional value, we currently can not guarantee these values will not be mutated
    //       by a VAR_INPUT {ref} function call.
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
            // This includes both Identifier and ReferenceExpr nodes for named arguments
            if let AstStatement::Identifier(_) | AstStatement::ReferenceExpr(_) = statement.get_stmt() {
                return;
            }
            validator.push_diagnostic(
                Diagnostic::new(format!("Could not resolve generic type {generic_symbol} with {nature}"))
                    .with_error_code("E064")
                    .with_location(statement),
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
                    Diagnostic::new(format!(
                        "Invalid type nature for generic argument. {} is no {}",
                        actual_type.get_name(),
                        generic_nature
                    ))
                    .with_error_code("E062")
                    .with_location(statement),
                );
            }
        }
    }
}

fn validate_assignment_type_sizes<T: AnnotationMap>(
    validator: &mut Validator,
    left: &DataType,
    right: &AstNode,
    context: &ValidationContext<T>,
) {
    fn get_expression_types_and_locations<'b, T: AnnotationMap>(
        expression: &AstNode,
        context: &'b ValidationContext<T>,
        lhs_is_signed_int: bool,
        is_builtin_call: bool,
    ) -> FxHashMap<&'b DataType, Vec<SourceLocation>> {
        let mut map: FxHashMap<&DataType, Vec<SourceLocation>> = FxHashMap::default();
        match expression.get_stmt_peeled() {
            AstStatement::BinaryExpression(BinaryExpression { operator, left, right, .. })
                if !operator.is_comparison_operator() =>
            {
                get_expression_types_and_locations(left, context, lhs_is_signed_int, false)
                    .into_iter()
                    .for_each(|(k, v)| map.entry(k).or_default().extend(v));
                // the RHS type in a MOD expression has no impact on the resulting value type
                if matches!(operator, Operator::Modulo) {
                    return map
                };
                get_expression_types_and_locations(right, context, lhs_is_signed_int, false)
                    .into_iter()
                    .for_each(|(k, v)| map.entry(k).or_default().extend(v));
            }
            AstStatement::UnaryExpression(UnaryExpression { operator, value })
                if !operator.is_comparison_operator() =>
            {
                get_expression_types_and_locations(value, context, lhs_is_signed_int, false)
                    .into_iter()
                    .for_each(|(k, v)| map.entry(k).or_default().extend(v));
            }
            // `get_literal_actual_signed_type_name` will always return `LREAL` for FP literals, so they will be handled by the fall-through case according to their annotated type
            AstStatement::Literal(lit) if !matches!(lit, &AstLiteral::Real(_)) => {
                if !lit.is_numerical() {
                    return map
                }
                if let Some(dt) = get_literal_actual_signed_type_name(lit, lhs_is_signed_int)
                    .map(|name| context.index.get_type(name).unwrap_or(context.index.get_void_type()))
                {
                    map.entry(dt).or_default().push(expression.get_location());
                }
            }
            AstStatement::CallStatement(CallStatement { operator, parameters })
                // special handling for builtin selector functions MUX and SEL
                if matches!(operator.get_flat_reference_name().unwrap_or_default(), "MUX" | "SEL") =>
            {
                let Some(args) = parameters else {
                    return map
                };
                if let AstStatement::ExpressionList(list) = args.get_stmt_peeled() {
                    // skip the selector argument since it will never be assigned to the target type
                    list.iter().skip(1).flat_map(|arg| {
                        get_expression_types_and_locations(arg, context, lhs_is_signed_int, true)
                    })
                    .for_each(|(k, v)| map.entry(k).or_default().extend(v));
                };
            }
            _ => {
                if !(context.annotations.get_generic_nature(expression).is_none() || is_builtin_call) {
                    return map
                };
                if let Some(dt) = context.annotations.get_type(expression, context.index) {
                    map.entry(dt).or_default().push(expression.get_location());
                }
            }
        };
        map
    }

    let lhs = left.get_type_information();
    let Ok(lhs_size) = lhs.get_size(context.index) else { return };
    let results_in_truncation = |rhs: &DataType| {
        let rhs = rhs.get_type_information();
        let Ok(rhs_size) = rhs.get_size(context.index) else { return false };
        lhs_size < rhs_size
            || (lhs_size == rhs_size
                && ((lhs.is_signed_int() && rhs.is_unsigned_int()) || (lhs.is_int() && rhs.is_float())))
    };

    get_expression_types_and_locations(right, context, lhs.is_signed_int(), false)
        .into_iter()
        .filter(|(dt, _)| !dt.is_aggregate_type() && results_in_truncation(dt))
        .for_each(|(dt, location)| {
            location.into_iter().for_each(|loc| {
                validator.push_diagnostic(
                    Diagnostic::new(format!(
                        "Implicit downcast from '{}' to '{}'.",
                        validator.get_type_name_or_slice(dt),
                        validator.get_type_name_or_slice(left)
                    ))
                    .with_error_code("E067")
                    .with_location(loc),
                );
            })
        });
}

/// Validates if a POU call has the correct number of arguments. Specifically, for functions,
/// the argument count must be equal to the required count unless the interface is variadic,
/// in which case the argument count may be greater than or equal to the required count. For stateful
/// POUs, the argument count can be less than or equal to the required count since VAR_INPUT and
/// VAR_OUTPUT arguments are optional.
fn validate_argument_count<T: AnnotationMap>(
    context: &ValidationContext<T>,
    validator: &mut Validator,
    pou: &PouIndexEntry,
    arguments: &[&AstNode],
    operator_location: &SourceLocation,
) {
    let parameters = context.index.get_available_parameters(pou.get_name());
    let has_variadic_parameter = context.index.has_variadic_parameter(pou.get_name());

    let argument_count_is_incorrect = match pou {
        PouIndexEntry::Function { .. } => {
            // parameters with default values are optional, so the argument count can be less than
            // the parameter count. This only works if the parameters with default values are at the end
            let optional_parameters =
                parameters.iter().rev().take_while(|p| p.initial_value.is_some()).count();
            let min_required_parameters = parameters.len() - optional_parameters;
            arguments.len() < min_required_parameters
                || (!has_variadic_parameter && arguments.len() > parameters.len())
        }

        PouIndexEntry::Program { .. } | PouIndexEntry::FunctionBlock { .. } => {
            arguments.len() > parameters.len() && !has_variadic_parameter
        }

        _ => false,
    };

    if argument_count_is_incorrect {
        validator.push_diagnostic(Diagnostic::invalid_argument_count(
            parameters.len(),
            arguments.len(),
            operator_location,
        ));
    }
}

pub(crate) mod helper {
    use std::ops::Range;

    use plc_ast::ast::{AstNode, DirectAccessType};

    use crate::index::VariableIndexEntry;
    use crate::resolver::AnnotationMap;
    use crate::validation::ValidationContext;
    use crate::{index::Index, typesystem::DataTypeInformation};

    /// Returns true if the current index is in the range for the given type
    pub fn is_in_range(
        access: &DirectAccessType,
        access_index: u64,
        data_type: &DataTypeInformation,
        index: &Index,
    ) -> bool {
        (access.get_bit_width() * access_index) < data_type.get_size_in_bits(index).unwrap_or_default() as u64
    }

    /// Returns the range from 0 for the given data type
    pub fn get_range(
        access: &DirectAccessType,
        data_type: &DataTypeInformation,
        index: &Index,
    ) -> Range<u64> {
        0..((data_type.get_size_in_bits(index).unwrap_or_default() as u64 / access.get_bit_width()) - 1)
    }

    /// Returns true if the direct access can be used for the given type
    pub fn is_compatible(access: &DirectAccessType, data_type: &DataTypeInformation, index: &Index) -> bool {
        data_type.get_semantic_size(index) as u64 > access.get_bit_width()
    }

    pub fn get_literal_int_or_const_expr_value<T>(
        right: &AstNode,
        context: &ValidationContext<T>,
    ) -> Option<i128>
    where
        T: AnnotationMap,
    {
        if let Some(value) = right.get_literal_integer_value() {
            return Some(value);
        }

        let path = right.get_flat_reference_name().unwrap_or_default();
        let element = context.index.find_variable(context.qualifier, &[path])?;

        context
            .index
            .get_const_expressions()
            .maybe_get_constant_statement(&element.initial_value)
            .and_then(AstNode::get_literal_integer_value)
    }

    pub fn get_enum_variant_values<'idx>(
        index: &'idx Index,
        variable: &VariableIndexEntry,
    ) -> Vec<(&'idx VariableIndexEntry, i128)> {
        let mut variant_const_values = Vec::new();
        for variant in index.get_enum_variants_by_variable(variable) {
            if let Some(ref const_id) = variant.initial_value {
                if let Ok(init) = index.get_const_expressions().get_constant_int_statement_value(const_id) {
                    variant_const_values.push((variant, init));
                }
            }
        }

        variant_const_values
    }
}
