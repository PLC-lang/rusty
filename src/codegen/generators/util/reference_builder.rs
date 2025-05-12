
use anyhow::{bail, Result};
use inkwell::values::{BasicValueEnum, PointerValue};
use plc_ast::ast::AstId;

#[derive(Debug)]
pub enum GeneratedValue<'ink> {
    RValue((BasicValueEnum<'ink>, AstId)),
    // LValue(PointerValue<'ink>),
    LValue((PointerValue<'ink>, AstId)),
    NoValue,
}

impl <'ink> GeneratedValue<'ink> {

    pub fn as_pointer_value(&self) -> Result<PointerValue<'ink>> {
        match self {
            GeneratedValue::LValue((pv, ..)) => Ok(*pv) ,
            _ => bail!("Expected LValue but got {:#?}", self),
        }
    }
}
