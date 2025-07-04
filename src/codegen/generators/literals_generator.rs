use anyhow::{anyhow, bail, Result};
use plc_ast::ast::AstNode;

use crate::{
    codegen::{
        generators::{expression_visitor::GeneratedValue, llvm::Llvm},
        llvm_index::LlvmTypedIndex,
    },
    index::Index,
    typesystem::DataTypeInformation,
};

pub struct LlvmLiteralsGenerator<'ink, 'idx> {
    llvm: &'ink Llvm<'ink>,
    llvm_index: &'ink LlvmTypedIndex<'ink>,
    index: &'idx Index,
}

impl<'ink, 'idx> LlvmLiteralsGenerator<'ink, 'idx> {
    /// Creates a new `LlvmLiteralsGenerator` instance.
    pub fn new(llvm: &'ink Llvm<'ink>, llvm_index: &'ink LlvmTypedIndex<'ink>, index: &'idx Index) -> Self {
        Self { llvm, llvm_index, index }
    }

    /// Generates a constant integer value for the given type and value.
    /// Returns a `BasicValueEnum` representing the constant integer value.
    pub fn generate_const_int(
        &self,
        t: &DataTypeInformation,
        v: i128,
        node: &AstNode,
    ) -> Result<GeneratedValue<'ink>> {
        let t = self.index.get_intrinsic_type_information(t);
        match t {
            DataTypeInformation::Integer { name, size, .. } => {
                let llvm_type = match size {
                    1 => self.llvm.context.i8_type(),
                    8 => self.llvm.context.i8_type(),
                    16 => self.llvm.context.i16_type(),
                    32 => self.llvm.context.i32_type(),
                    64 => self.llvm.context.i64_type(),
                    _ => {
                        bail!("Unsupported size {size} for type {name}")
                    }
                };
                let value = llvm_type.const_int(v as u64, false);
                Ok(GeneratedValue::RValue((value.into(), node.get_id())))
            }
            DataTypeInformation::Enum { referenced_type, .. } => {
                let ref_type = self
                    .index
                    .find_effective_type_info(&referenced_type)
                    .ok_or_else(|| anyhow!("Cannot find type {referenced_type}"))?;
                self.generate_const_int(ref_type, v, node)
            }
            _ => bail!("Expected IntType but got: {t:?}"),
        }
    }

    /// Generates a constant float value for the given type and value.
    /// Returns a `BasicValueEnum` representing the constant float value.
    pub fn generate_const_float(
        &self,
        t: &DataTypeInformation,
        v: f64,
        node: &AstNode,
    ) -> Result<GeneratedValue<'ink>> {
        if let DataTypeInformation::Float { name, size, .. } = t {
            let llvm_type = match size {
                32 => self.llvm.context.f32_type(),
                64 => self.llvm.context.f64_type(),
                _ => bail!("Unsupported size {size} for type {name}"),
            };
            let value = llvm_type.const_float(v);
            Ok(GeneratedValue::RValue((value.into(), node.get_id())))
        } else {
            bail!("Expected FloatType but got: {t:?}")
        }
    }

    /// Generates a constant string value for the given type and value.
    /// Returns a `BasicValueEnum` representing the constant string value.
    pub fn generate_const_string(
        &self,
        t: &DataTypeInformation,
        value: &str,
        node: &AstNode,
    ) -> Result<GeneratedValue<'ink>> {
        match t {
            DataTypeInformation::String { encoding, .. } => {
                //todo: add documentation
                let v = if *encoding == crate::typesystem::StringEncoding::Utf8 {
                    self.llvm_index.find_utf08_literal_string(value)
                } else {
                    self.llvm_index.find_utf16_literal_string(value)
                }
                .ok_or_else(|| anyhow!("Cannot find string literal '{value}'"))?;

                Ok(GeneratedValue::LValue((v.as_pointer_value(), node.get_id())))
            }
            DataTypeInformation::Integer { size: 8, .. } if t.is_character() => {
                let value = self.llvm.create_llvm_const_i8_char(value, &node.location)?;
                Ok(GeneratedValue::RValue((value.into(), node.get_id())))
            }
            DataTypeInformation::Integer { size: 16, .. } if t.is_character() => {
                let value = self.llvm.create_llvm_const_i16_char(value, &node.location)?;
                Ok(GeneratedValue::RValue((value.into(), node.get_id())))
            }
            _ => bail!("Expected StringType but got: {t:?}"),
        }
    }
}
