use std::{convert::TryInto, mem::discriminant};

use super::{ValidationContext, Validators};
use crate::{
    ast::{AstStatement, DirectAccessType, Operator, SourceRange},
    index::{ArgumentType, VariableIndexEntry, VariableType},
    resolver::{AnnotationMap, StatementAnnotation},
    typesystem::{
        DataType, DataTypeInformation, Dimension, BOOL_TYPE, DATE_AND_TIME_TYPE, DATE_TYPE, DINT_TYPE,
        INT_TYPE, LINT_TYPE, LREAL_TYPE, POINTER_SIZE, SINT_TYPE, STRING_TYPE, TIME_OF_DAY_TYPE, TIME_TYPE,
        UDINT_TYPE, UINT_TYPE, ULINT_TYPE, USINT_TYPE, VOID_TYPE, WSTRING_TYPE,
    },
    Diagnostic,
};

/// validates control-statements, assignments

//returns a range with the min and max value of the given type
macro_rules! is_covered_by {
    ($t:ty, $e:expr) => {
        <$t>::MIN as i128 <= $e as i128 && $e as i128 <= <$t>::MAX as i128
    };
}

#[derive(Default)]
pub struct StatementValidator {
    diagnostics: Vec<Diagnostic>,
}

impl Validators for StatementValidator {
    fn push_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn take_diagnostics(&mut self) -> Vec<Diagnostic> {
        std::mem::take(&mut self.diagnostics)
    }
}

impl StatementValidator {
    pub fn new() -> StatementValidator {
        StatementValidator { diagnostics: Vec::new() }
    }

    pub fn validate_statement(&mut self, statement: &AstStatement, context: &ValidationContext) {
        match statement {
            AstStatement::Reference { name, location, .. } => {
                self.validate_reference(statement, name, location, context);
            }

            AstStatement::UnaryExpression { operator, value, location, .. } => {
                if operator == &Operator::Address {
                    match value.as_ref() {
                        AstStatement::Reference { .. }
                        | AstStatement::QualifiedReference { .. }
                        | AstStatement::ArrayAccess { .. } => (),

                        _ => self.push_diagnostic(Diagnostic::invalid_operation(
                            "Invalid address-of operation",
                            location.to_owned(),
                        )),
                    }
                }
            }

            AstStatement::CastStatement { location, target, type_name, .. } => {
                self.validate_cast_literal(target, type_name, location, context);
            }
            AstStatement::ArrayAccess { reference, access, .. } => {
                let target_type =
                    context.ast_annotation.get_type_or_void(reference, context.index).get_type_information();

                if let DataTypeInformation::Array { dimensions, .. } = target_type {
                    if let AstStatement::ExpressionList { expressions, .. } = access.as_ref() {
                        for (i, exp) in expressions.iter().enumerate() {
                            self.validate_array_access(exp, dimensions, i, context);
                        }
                    } else {
                        self.validate_array_access(access.as_ref(), dimensions, 0, context);
                    }
                } else {
                    self.push_diagnostic(Diagnostic::incompatible_array_access_variable(
                        target_type.get_name(),
                        access.get_location(),
                    ));
                }
            }
            AstStatement::QualifiedReference { elements, .. } => {
                let mut i = elements.iter().rev();
                if let Some((AstStatement::DirectAccess { access, index, location, .. }, reference)) =
                    i.next().zip(i.next())
                {
                    let target_type = context
                        .ast_annotation
                        .get_type_or_void(reference, context.index)
                        .get_type_information();
                    if target_type.is_int() {
                        if !access.is_compatible(target_type, context.index) {
                            self.push_diagnostic(Diagnostic::incompatible_directaccess(
                                &format!("{access:?}"),
                                access.get_bit_width(),
                                location.clone(),
                            ))
                        } else {
                            self.validate_access_index(context, index, access, target_type, location);
                        }
                    } else {
                        //Report incompatible type issue
                        self.push_diagnostic(Diagnostic::incompatible_directaccess(
                            &format!("{access:?}"),
                            access.get_bit_width(),
                            location.clone(),
                        ))
                    }
                }
            }
            AstStatement::Assignment { left, right, .. } => {
                if let Some(StatementAnnotation::Variable {
                    constant,
                    qualified_name: l_qualified_name,
                    resulting_type: l_resulting_type,
                    ..
                }) = context.ast_annotation.get(left.as_ref())
                {
                    // check if we assign to a constant variable
                    if *constant {
                        self.push_diagnostic(Diagnostic::cannot_assign_to_constant(
                            l_qualified_name.as_str(),
                            left.get_location(),
                        ));
                    }

                    let l_effective_type = context
                        .index
                        .get_effective_type_or_void_by_name(l_resulting_type)
                        .get_type_information();
                    let r_effective_type =
                        context.ast_annotation.get_type_or_void(right, context.index).get_type_information();

                    //check if Datatype can hold a Pointer (u64)
                    if r_effective_type.is_pointer()
                        && !l_effective_type.is_pointer()
                        && l_effective_type.get_size_in_bits(context.index) < POINTER_SIZE
                    {
                        self.push_diagnostic(Diagnostic::incompatible_type_size(
                            l_effective_type.get_name(),
                            l_effective_type.get_size_in_bits(context.index),
                            "hold a",
                            statement.get_location(),
                        ));
                    }
                    //check if size allocated to Pointer is standart pointer size (u64)
                    else if l_effective_type.is_pointer()
                        && !r_effective_type.is_pointer()
                        && r_effective_type.get_size_in_bits(context.index) < POINTER_SIZE
                    {
                        self.push_diagnostic(Diagnostic::incompatible_type_size(
                            r_effective_type.get_name(),
                            r_effective_type.get_size_in_bits(context.index),
                            "to be stored in a",
                            statement.get_location(),
                        ));
                    }

                    // valid assignments -> char := literalString, char := char
                    // check if we assign to a character variable -> char := ..
                    if l_effective_type.is_character() {
                        if let AstStatement::LiteralString { value, location, .. } = right.as_ref() {
                            // literalString may only be 1 character long
                            if value.len() > 1 {
                                self.push_diagnostic(Diagnostic::syntax_error(
                                    format!("Value: '{value}' exceeds length for type: {l_resulting_type}",)
                                        .as_str(),
                                    location.clone(),
                                ));
                            }
                        } else if l_effective_type != r_effective_type {
                            // invalid assignment
                            self.push_diagnostic(Diagnostic::invalid_assignment(
                                r_effective_type.get_name(),
                                l_effective_type.get_name(),
                                statement.get_location(),
                            ));
                        }
                    } else if r_effective_type.is_character() {
                        // if we try to assign a character variable -> .. := char
                        // and didn't match the first if, left and right won't have the same type -> invalid assignment
                        self.push_diagnostic(Diagnostic::invalid_assignment(
                            r_effective_type.get_name(),
                            l_effective_type.get_name(),
                            statement.get_location(),
                        ));
                    }
                }
            }
            AstStatement::BinaryExpression { operator, left, right, .. } => match operator {
                Operator::NotEqual => {
                    self.validate_binary_expression(context, &Operator::Equal, left, right, statement)
                }
                Operator::GreaterOrEqual => {
                    //check for the > operator
                    self.validate_binary_expression(context, &Operator::Greater, left, right, statement);
                    //check for the = operator
                    self.validate_binary_expression(context, &Operator::Equal, left, right, statement);
                }
                Operator::LessOrEqual => {
                    //check for the < operator
                    self.validate_binary_expression(context, &Operator::Less, left, right, statement);
                    //check for the = operator
                    self.validate_binary_expression(context, &Operator::Equal, left, right, statement);
                }
                _ => self.validate_binary_expression(context, operator, left, right, statement),
            },
            _ => (),
        }
        self.validate_type_nature(statement, context);
    }

    /// Validates that the assigned type and type hint are compatible with the nature for this
    /// statement
    fn validate_type_nature(&mut self, statement: &AstStatement, context: &ValidationContext) {
        if let Some(statement_type) = context
            .ast_annotation
            .get_type_hint(statement, context.index)
            .or_else(|| context.ast_annotation.get_type(statement, context.index))
        {
            if let DataTypeInformation::Generic { generic_symbol, nature, .. } =
                statement_type.get_type_information()
            {
                self.push_diagnostic(Diagnostic::unresolved_generic_type(
                    generic_symbol,
                    &format!("{nature:?}"),
                    statement.get_location(),
                ))
            } else if let Some((actual_type, generic_nature)) = context
                .ast_annotation
                .get_type(statement, context.index)
                .zip(context.ast_annotation.get_generic_nature(statement))
            {
                if !statement_type.has_nature(actual_type.nature, context.index)
				// INT parameter for REAL is allowed
                    & !(statement_type.is_real() & actual_type.is_numerical())
                {
                    self.push_diagnostic(Diagnostic::invalid_type_nature(
                        actual_type.get_name(),
                        format!("{generic_nature:?}").as_str(),
                        statement.get_location(),
                    ));
                }
            }
        }
    }

    fn validate_access_index(
        &mut self,
        context: &ValidationContext,
        access_index: &AstStatement,
        access_type: &DirectAccessType,
        target_type: &DataTypeInformation,
        location: &SourceRange,
    ) {
        match *access_index {
            AstStatement::LiteralInteger { value, .. } => {
                if !access_type.is_in_range(value.try_into().unwrap_or_default(), target_type, context.index)
                {
                    self.push_diagnostic(Diagnostic::incompatible_directaccess_range(
                        &format!("{access_type:?}"),
                        target_type.get_name(),
                        access_type.get_range(target_type, context.index),
                        location.clone(),
                    ))
                }
            }
            AstStatement::Reference { .. } => {
                let ref_type = context.ast_annotation.get_type_or_void(access_index, context.index);
                if !ref_type.get_type_information().is_int() {
                    self.push_diagnostic(Diagnostic::incompatible_directaccess_variable(
                        ref_type.get_name(),
                        location.clone(),
                    ))
                }
            }
            _ => unreachable!(),
        }
    }

    fn validate_array_access(
        &mut self,
        access: &AstStatement,
        dimensions: &[Dimension],
        dimension_index: usize,
        context: &ValidationContext,
    ) {
        if let AstStatement::LiteralInteger { value, .. } = access {
            let dimension = dimensions.get(dimension_index);
            if let Some(dimension) = dimension {
                let range = dimension.get_range(context.index);
                if let Ok(range) = range {
                    if !(range.start as i128 <= *value && range.end as i128 >= *value) {
                        self.diagnostics
                            .push(Diagnostic::incompatible_array_access_range(range, access.get_location()))
                    }
                }
            }
        } else {
            let type_info =
                context.ast_annotation.get_type_or_void(access, context.index).get_type_information();
            if !type_info.is_int() {
                self.push_diagnostic(Diagnostic::incompatible_array_access_type(
                    type_info.get_name(),
                    access.get_location(),
                ))
            }
        }
    }

    fn validate_reference(
        &mut self,
        statement: &AstStatement,
        ref_name: &str,
        location: &SourceRange,
        context: &ValidationContext,
    ) {
        // unresolved reference
        if !context.ast_annotation.has_type_annotation(statement) {
            self.push_diagnostic(Diagnostic::unresolved_reference(ref_name, location.clone()));
        } else if let Some(StatementAnnotation::Variable { qualified_name, variable_type, .. }) =
            context.ast_annotation.get(statement)
        {
            //check if we're accessing a private variable AND the variable's qualifier is not the
            //POU we're accessing it from
            if variable_type.is_private()
                && context
                    .qualifier
                    .and_then(|it| context.index.find_pou(it)) //Get the container pou (for actions this is the program/fb)
                    .map(|it| (it.get_name(), it.get_container()))
                    .map_or(false, |(it, container)| {
                        !qualified_name.starts_with(it) && !qualified_name.starts_with(container)
                    })
            {
                self.push_diagnostic(Diagnostic::illegal_access(qualified_name.as_str(), location.clone()));
            }
        }
    }

    /// validates a literal statement with a dedicated type-prefix (e.g. INT#3)
    ///
    /// checks whether ...
    /// - the type-prefix is valid
    fn validate_cast_literal(
        &mut self,
        literal: &AstStatement,
        type_name: &str,
        location: &SourceRange,
        context: &ValidationContext,
    ) {
        let cast_type = context.index.get_effective_type_or_void_by_name(type_name).get_type_information();

        let literal_type = context
            .index
            .find_effective_type_info(
                StatementValidator::get_literal_actual_signed_type_name(
                    literal,
                    !cast_type.is_unsigned_int(),
                )
                .or_else(|| {
                    context.ast_annotation.get_type_hint(literal, context.index).map(DataType::get_name)
                })
                .unwrap_or_else(|| {
                    context.ast_annotation.get_type_or_void(literal, context.index).get_name()
                }),
            )
            .unwrap_or_else(|| context.index.get_void_type().get_type_information());

        if !is_typable_literal(literal) {
            self.push_diagnostic(Diagnostic::literal_expected(location.clone()))
        } else if is_date_or_time_type(cast_type) || is_date_or_time_type(literal_type) {
            self.push_diagnostic(Diagnostic::incompatible_literal_cast(
                cast_type.get_name(),
                literal_type.get_name(),
                location.clone(),
            ));
            //see if target and cast_type are compatible
        } else if cast_type.is_int() && literal_type.is_int() {
            //INTs with INTs
            if cast_type.get_semantic_size(context.index) < literal_type.get_semantic_size(context.index) {
                self.push_diagnostic(Diagnostic::literal_out_of_range(
                    StatementValidator::get_literal_value(literal).as_str(),
                    cast_type.get_name(),
                    location.clone(),
                ));
            }
        } else if cast_type.is_character() && literal_type.is_string() {
            let value = StatementValidator::get_literal_value(literal);
            // value contains "" / ''
            if value.len() > 3 {
                self.push_diagnostic(Diagnostic::literal_out_of_range(
                    value.as_str(),
                    cast_type.get_name(),
                    location.clone(),
                ));
            }
        } else if discriminant(cast_type) != discriminant(literal_type) {
            // different types
            // REAL#100 is fine, other differences are not
            if !(cast_type.is_float() && literal_type.is_int()) {
                self.push_diagnostic(Diagnostic::incompatible_literal_cast(
                    cast_type.get_name(),
                    StatementValidator::get_literal_value(literal).as_str(),
                    location.clone(),
                ));
            }
        }
    }

    fn get_literal_value(literal: &AstStatement) -> String {
        match literal {
            AstStatement::LiteralString { value, is_wide: true, .. } => format!(r#""{value}""#),
            AstStatement::LiteralString { value, is_wide: false, .. } => format!(r#"'{value}'"#),
            AstStatement::LiteralBool { value, .. } => {
                format!("{value}")
            }
            AstStatement::LiteralInteger { value, .. } => {
                format!("{value}")
            }
            AstStatement::LiteralReal { value, .. } => value.clone(),
            _ => {
                format!("{literal:#?}")
            }
        }
    }

    fn get_literal_actual_signed_type_name(target: &AstStatement, signed: bool) -> Option<&str> {
        match target {
            AstStatement::LiteralInteger { value, .. } => match signed {
                _ if *value == 0_i128 || *value == 1_i128 => Some(BOOL_TYPE),
                true if is_covered_by!(i8, *value) => Some(SINT_TYPE),
                true if is_covered_by!(i16, *value) => Some(INT_TYPE),
                true if is_covered_by!(i32, *value) => Some(DINT_TYPE),
                true if is_covered_by!(i64, *value) => Some(LINT_TYPE),

                false if is_covered_by!(u8, *value) => Some(USINT_TYPE),
                false if is_covered_by!(u16, *value) => Some(UINT_TYPE),
                false if is_covered_by!(u32, *value) => Some(UDINT_TYPE),
                false if is_covered_by!(u64, *value) => Some(ULINT_TYPE),
                _ => Some(VOID_TYPE),
            },
            AstStatement::LiteralBool { .. } => Some(BOOL_TYPE),
            AstStatement::LiteralString { is_wide: true, .. } => Some(WSTRING_TYPE),
            AstStatement::LiteralString { is_wide: false, .. } => Some(STRING_TYPE),
            AstStatement::LiteralReal { .. } => Some(LREAL_TYPE),
            AstStatement::LiteralDate { .. } => Some(DATE_TYPE),
            AstStatement::LiteralDateAndTime { .. } => Some(DATE_AND_TIME_TYPE),
            AstStatement::LiteralTime { .. } => Some(TIME_TYPE),
            AstStatement::LiteralTimeOfDay { .. } => Some(TIME_OF_DAY_TYPE),
            _ => None,
        }
    }

    /// checks if the given binary expression is valid
    fn validate_binary_expression(
        &mut self,
        context: &ValidationContext,
        operator: &Operator,
        left: &AstStatement,
        right: &AstStatement,
        binary_statement: &AstStatement,
    ) {
        let left_type = context.ast_annotation.get_type_or_void(left, context.index).get_type_information();
        let right_type = context.ast_annotation.get_type_or_void(right, context.index).get_type_information();

        // if the type is a subrange, check if the intrinsic type is numerical
        let is_numerical = context.index.find_intrinsic_type(left_type).is_numerical();

        if std::mem::discriminant(left_type) == std::mem::discriminant(right_type)
            && !(is_numerical || left_type.is_pointer())
        {
            //see if we have the right compare-function (non-numbers are compared using user-defined callback-functions)
            if operator.is_comparison_operator()
                && !compare_function_exists(left_type.get_name(), operator, context)
            {
                self.push_diagnostic(Diagnostic::missing_compare_function(
                    crate::typesystem::get_equals_function_name_for(left_type.get_name(), operator)
                        .unwrap_or_default()
                        .as_str(),
                    left_type.get_name(),
                    binary_statement.get_location(),
                ));
            }
        }
    }
}

/// returns true if the index contains a compare function for the given operator and type
fn compare_function_exists(type_name: &str, operator: &Operator, context: &ValidationContext) -> bool {
    let implementation = crate::typesystem::get_equals_function_name_for(type_name, operator)
        .as_ref()
        .and_then(|function_name| context.index.find_pou_implementation(function_name));

    if let Some(implementation) = implementation {
        let members = context.index.get_pou_members(implementation.get_type_name());

        //we expect two input parameters and a return-parameter
        if let [VariableIndexEntry {
            data_type_name: type_name_1,
            variable_type: ArgumentType::ByVal(VariableType::Input),
            ..
        }, VariableIndexEntry {
            data_type_name: type_name_2,
            variable_type: ArgumentType::ByVal(VariableType::Input),
            ..
        }, VariableIndexEntry {
            data_type_name: return_type,
            variable_type: ArgumentType::ByVal(VariableType::Return),
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

            //both parameters must have the same type and the return type must be BOOL
            if type_name_1 == type_name && type_name_2 == type_name && return_type == BOOL_TYPE {
                return true;
            }
        }
    }

    false
}

fn is_date_or_time_type(cast_type: &crate::typesystem::DataTypeInformation) -> bool {
    return cast_type.get_name() == DATE_TYPE
        || cast_type.get_name() == DATE_AND_TIME_TYPE
        || cast_type.get_name() == TIME_OF_DAY_TYPE
        || cast_type.get_name() == TIME_TYPE;
}

/// returns true if this AST Statement is a literal that can be
/// prefixed with a type-cast (e.g. INT#23)
fn is_typable_literal(literal: &AstStatement) -> bool {
    matches!(
        literal,
        AstStatement::LiteralBool { .. }
            | AstStatement::LiteralInteger { .. }
            | AstStatement::LiteralReal { .. }
            | AstStatement::LiteralString { .. }
            | AstStatement::LiteralTime { .. }
            | AstStatement::LiteralDate { .. }
            | AstStatement::LiteralTimeOfDay { .. }
            | AstStatement::LiteralDateAndTime { .. }
            | AstStatement::Reference { .. }
    )
}
