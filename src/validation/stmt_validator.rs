use crate::{ast::{Statement}, index::Index};
use super::ValidationContext;

/// validates control-statements, assignments

pub struct StatementValidator<'i> {
    index: &'i Index,
}

impl<'i> StatementValidator<'i> {
    pub fn new(index: &'i Index) -> StatementValidator {
        StatementValidator {
            index
        }
    }

    pub fn validate_statement(&self, statement: &Statement, da: &mut ValidationContext) {
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
            // Statement::Reference { name, location } => todo!(),
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
            _=> {}
        }
    }

}
 