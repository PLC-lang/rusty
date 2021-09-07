use std::mem::discriminant;

use super::ValidationContext;
use crate::{
    ast::{AstStatement, SourceRange},
    typesystem::{
        BOOL_TYPE, DATE_AND_TIME_TYPE, DATE_TYPE, DINT_TYPE, INT_TYPE, LINT_TYPE, LREAL_TYPE,
        SINT_TYPE, STRING_TYPE, TIME_OF_DAY_TYPE, TIME_TYPE, UDINT_TYPE, UINT_TYPE, ULINT_TYPE,
        USINT_TYPE, VOID_TYPE, WSTRING_TYPE,
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

pub struct StatementValidator {
    pub diagnostics: Vec<Diagnostic>,
}

impl StatementValidator {
    pub fn new() -> StatementValidator {
        StatementValidator {
            diagnostics: Vec::new(),
        }
    }

    pub fn validate_statement(&mut self, statement: &AstStatement, context: &ValidationContext) {
        match statement {
            AstStatement::Reference {
                name, location, id, ..
            } => {
                self.validate_reference(id, name, location, context);
            }
            AstStatement::CastStatement {
                location,
                target,
                type_name,
                ..
            } => {
                self.validate_cast_literal(target, type_name, location, context);
            }
            AstStatement::QualifiedReference { elements, .. } => {
                if elements.len() > 1 {
                    if let Some(AstStatement::DirectAccess {
                        access,
                        index,
                        location,
                        ..
                    }) = elements.last()
                    {
                        if let Some(reference) =
                            elements.split_last().and_then(|(_, map)| map.last())
                        {
                            let target_type = context
                                .ast_annotation
                                .get_type_or_void(reference, context.index)
                                .get_type_information();
                            if target_type.is_int() {
                                if !access.is_compatible(target_type) {
                                    self.diagnostics.push(Diagnostic::incompatible_directaccess(
                                        &format!("{:?}", access),
                                        access.get_bit_witdh(),
                                        location.clone(),
                                    ))
                                } else if !access.is_in_range(*index, target_type) {
                                    self.diagnostics.push(
                                        Diagnostic::incompatible_directaccess_range(
                                            &format!("{:?}", access),
                                            target_type.get_name(),
                                            access.get_range(target_type),
                                            location.clone(),
                                        ),
                                    )
                                }
                            } else {
                                //Report incompatible type issue
                                self.diagnostics.push(Diagnostic::incompatible_directaccess(
                                    &format!("{:?}", access),
                                    access.get_bit_witdh(),
                                    location.clone(),
                                ))
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }

    fn validate_reference(
        &mut self,
        id: &usize,
        ref_name: &str,
        location: &SourceRange,
        context: &ValidationContext,
    ) {
        if !context.ast_annotation.has_type_annotation(id) {
            self.diagnostics
                .push(Diagnostic::unrseolved_reference(ref_name, location.clone()));
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
        let cast_type = context
            .index
            .get_effective_type_by_name(type_name)
            .get_type_information();

        let literal_type = context.index.get_type_information_or_void(
            StatementValidator::get_literal_actual_signed_type_name(
                literal,
                !cast_type.is_unsigned_int(),
            )
            .unwrap_or_else(|| {
                context
                    .ast_annotation
                    .get_type_or_void(literal, context.index)
                    .get_name()
            }),
        );
        if !is_typable_literal(literal) {
            self.diagnostics
                .push(Diagnostic::literal_expected(location.clone()))
        } else if is_date_or_time_type(cast_type) || is_date_or_time_type(literal_type) {
            self.diagnostics.push(Diagnostic::incompatible_literal_cast(
                cast_type.get_name(),
                literal_type.get_name(),
                location.clone(),
            ));
            //see if target and cast_type are compatible
        } else if cast_type.is_int() && literal_type.is_int() {
            //INTs with INTs
            if cast_type.get_size() < literal_type.get_size() {
                self.diagnostics.push(Diagnostic::literal_out_of_range(
                    StatementValidator::get_literal_value(literal).as_str(),
                    cast_type.get_name(),
                    location.clone(),
                ));
            }
        } else if discriminant(cast_type) != discriminant(literal_type) {
            // different types
            // REAL#100 is fine, other differences are not
            if !(cast_type.is_float() && literal_type.is_int()) {
                self.diagnostics.push(Diagnostic::incompatible_literal_cast(
                    cast_type.get_name(),
                    StatementValidator::get_literal_value(literal).as_str(),
                    location.clone(),
                ));
            }
        }
    }

    fn get_literal_value(literal: &AstStatement) -> String {
        match literal {
            AstStatement::LiteralString {
                value,
                is_wide: true,
                ..
            } => format!(r#""{:}""#, value),
            AstStatement::LiteralString {
                value,
                is_wide: false,
                ..
            } => format!(r#"'{:}'"#, value),
            AstStatement::LiteralBool { value, .. } => {
                format!("{}", value)
            }
            AstStatement::LiteralInteger { value, .. } => {
                format!("{}", value)
            }
            AstStatement::LiteralReal { value, .. } => value.clone(),
            _ => {
                format!("{:#?}", literal)
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
