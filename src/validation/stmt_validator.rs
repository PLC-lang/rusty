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
            ),
        );
        //see if target and cast_type are compatible
        if !is_typable_literal(literal) {
            self.diagnostics
                .push(Diagnostic::literal_expected(location.clone()))
        } else if is_date_or_time_type(cast_type) || is_date_or_time_type(literal_type) {
            self.diagnostics.push(Diagnostic::incompatible_literal_cast(
                cast_type.get_name(),
                literal_type.get_name(),
                location.clone(),
            ));
        } else if cast_type.is_int() && literal_type.is_int() {
            //INTs with INTs
            if cast_type.get_size() < literal_type.get_size() {
                self.diagnostics.push(Diagnostic::literal_out_of_range(
                    StatementValidator::get_literal_value(literal).as_str(),
                    cast_type.get_name(),
                    location.clone(),
                ));
            }
        } else if cast_type.is_int() && !literal_type.is_int() {
            // INTs with non-INTs
            self.diagnostics.push(Diagnostic::literal_out_of_range(
                StatementValidator::get_literal_value(literal).as_str(),
                cast_type.get_name(),
                location.clone(),
            ));
        } else if discriminant(cast_type) != discriminant(literal_type) {
            // different types
            // REAL#100 is fine, other differences are not
            if !(cast_type.is_float() && literal_type.is_int()) {
                self.diagnostics.push(Diagnostic::incompatible_literal_cast(
                    cast_type.get_name(),
                    literal_type.get_name(),
                    location.clone(),
                ));
            }
        }
    }

    fn get_literal_value(literal: &AstStatement) -> String {
        match literal {
            AstStatement::LiteralString { value, .. } => value.clone(),
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

    fn get_literal_actual_signed_type_name(target: &AstStatement, signed: bool) -> &str {
        match target {
            AstStatement::LiteralInteger { value, .. } => match signed {
                _ if *value == 0_i64 || *value == 1_i64 => BOOL_TYPE,
                true if is_covered_by!(i8, *value) => SINT_TYPE,
                true if is_covered_by!(i16, *value) => INT_TYPE,
                true if is_covered_by!(i32, *value) => DINT_TYPE,
                true if is_covered_by!(i64, *value) => LINT_TYPE,

                false if is_covered_by!(u8, *value) => USINT_TYPE,
                false if is_covered_by!(u16, *value) => UINT_TYPE,
                false if is_covered_by!(u32, *value) => UDINT_TYPE,
                false if is_covered_by!(u64, *value) => ULINT_TYPE,
                _ => VOID_TYPE,
            },
            AstStatement::LiteralBool { .. } => BOOL_TYPE,
            AstStatement::LiteralString { is_wide: true, .. } => WSTRING_TYPE,
            AstStatement::LiteralString { is_wide: false, .. } => STRING_TYPE,
            AstStatement::LiteralReal { .. } => LREAL_TYPE,
            AstStatement::LiteralDate { .. } => DATE_TYPE,
            AstStatement::LiteralDateAndTime { .. } => DATE_AND_TIME_TYPE,
            AstStatement::LiteralTime { .. } => TIME_TYPE,
            AstStatement::LiteralTimeOfDay { .. } => TIME_OF_DAY_TYPE,
            _ => VOID_TYPE,
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
    )
}
