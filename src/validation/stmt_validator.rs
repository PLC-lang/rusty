use chrono::format;

use super::ValidationContext;
use crate::{
    ast::{SourceRange, Statement},
    index::Index,
    Diagnostic,
};

/// validates control-statements, assignments

pub struct StatementValidator<'i> {
    index: &'i Index,
    pub diagnostics: Vec<Diagnostic>,
}

impl<'i> StatementValidator<'i> {
    pub fn new(index: &'i Index) -> StatementValidator {
        StatementValidator { index , diagnostics: Vec::new() }
    }

    pub fn validate_statement(&mut self, statement: &Statement, context: &ValidationContext) {
        match statement {
            // Statement::LiteralInteger { value, location } => todo!(),
            // Statement::LiteralDate { year, month, day, location } => todo!(),
            // Statement::LiteralDateAndTime { year, month, day, hour, min, sec, milli, location } => todo!(),
            // Statement::LiteralTimeOfDay { hour, min, sec, milli, location } => todo!(),
            // Statement::LiteralTime { day, hour, min, sec, milli, micro, nano, negative, location } => todo!(),
            // Statement::LiteralReal { value, location } => todo!(),
            // Statement::LiteralBool { value, location } => todo!(),
            // Statement::LiteralString { value, is_wide, location } => todo!(),
            // Statement::LiteralArray { elements, location } => todo!(),
            // Statement::MultipliedStatement { multiplier, element, location } => todo!(),
            // Statement::QualifiedReference { elements } => todo!(),
            Statement::Reference { name, location, id, .. } => {
                self.validate_reference(id, name, location, context);
            }
            // Statement::ArrayAccess { reference, access } => todo!(),
            // Statement::BinaryExpression { operator, left, right } => todo!(),
            // Statement::UnaryExpression { operator, value, location } => todo!(),
            // Statement::ExpressionList { expressions } => todo!(),
            // Statement::RangeStatement { start, end } => todo!(),
            // Statement::Assignment { left, right } => todo!(),
            // Statement::OutputAssignment { left, right } => todo!(),
            // Statement::CallStatement { operator, parameters, location } => todo!(),
            // Statement::IfStatement { blocks, else_block, location } => todo!(),
            // Statement::ForLoopStatement { counter, start, end, by_step, body, location } => todo!(),
            // Statement::WhileLoopStatement { condition, body, location } => todo!(),
            // Statement::RepeatLoopStatement { condition, body, location } => todo!(),
            // Statement::CaseStatement { selector, case_blocks, else_block, location } => todo!(),
            // Statement::CaseCondition { condition } => todo!(),
            // Statement::EmptyStatement { location } => todo!(),
            _ => {}
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
            self.diagnostics.push(Diagnostic::unrseolved_reference(ref_name, location.clone()));
        }
    }
}

#[cfg(test)]
mod statement_validation_tests {
    use crate::{validation::validation_tests::parse_and_validate, Diagnostic};

    #[test]
    fn validate_reference() {
        let diagnostics = parse_and_validate(
            "
            VAR_GLOBAL
                ga : INT;
            END_VAR

            PROGRAM prg
                VAR a : INT; END_VAR

                a;
                b;
                ga;
                gb;
                foo(a);
                boo(c);
                foo(x := a);
                foo(x := c);
                foo(y := a);

            END_PROGRAM

            FUNCTION foo : INT
                VAR_INPUT x : INT; END_VAR
            END_FUNCTION
        ",
        );

        assert_eq!(
            diagnostics,
            vec![
                Diagnostic::unrseolved_reference("b", (168..169).into()),
                Diagnostic::unrseolved_reference("gb", (207..209).into()),
                Diagnostic::unrseolved_reference("boo", (251..254).into()),
                Diagnostic::unrseolved_reference("c", (255..256).into()),
                Diagnostic::unrseolved_reference("c", (313..314).into()),
                Diagnostic::unrseolved_reference("y", (337..338).into()),
            ]
        );
    }
}
