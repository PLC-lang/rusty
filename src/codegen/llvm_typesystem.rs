// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use inkwell::{
    context::Context,
    types::{FloatType, IntType},
    values::{BasicValueEnum, FloatValue, IntValue, PointerValue},
};

use crate::{
    ast::SourceRange,
    diagnostics::Diagnostic,
    index::Index,
    typesystem::{DataType, DataTypeInformation},
};

use super::{generators::llvm::Llvm, llvm_index::LlvmTypedIndex};

/// generates a cast from the given `value` to the given `target_type` if necessary and returns the casted value. It returns
/// the original `value` if no cast is necessary
///
/// - `llvm` the llvm utilities to use for code-generation
/// - `index` the current Index used for type-lookups
/// - `llvm_type_index` the type index to lookup llvm generated types
/// - `target_type` the expected target type of the value
/// - `value` the value to (maybe) cast
/// - `value_type` the current type of the given value
pub fn cast_if_needed<'ctx>(
    llvm: &Llvm<'ctx>,
    index: &Index,
    llvm_type_index: &LlvmTypedIndex<'ctx>,
    target_type: &DataType,
    value: BasicValueEnum<'ctx>,
    value_type: &DataType,
) -> Result<BasicValueEnum<'ctx>, Diagnostic> {
    CastInstructionBuilder::generate_cast_instruction(
        llvm,
        index,
        llvm_type_index,
        value,
        value_type,
        target_type,
    )
}

pub fn get_llvm_int_type<'a>(context: &'a Context, size: u32, name: &str) -> Result<IntType<'a>, Diagnostic> {
    match size {
        1 => Ok(context.bool_type()),
        8 => Ok(context.i8_type()),
        16 => Ok(context.i16_type()),
        32 => Ok(context.i32_type()),
        64 => Ok(context.i64_type()),
        128 => Ok(context.i128_type()),
        _ => Err(Diagnostic::codegen_error(
            &format!("Invalid size for type : '{name}' at {size}"),
            SourceRange::undefined(),
        )),
    }
}

pub fn get_llvm_float_type<'a>(
    context: &'a Context,
    size: u32,
    name: &str,
) -> Result<FloatType<'a>, Diagnostic> {
    match size {
        32 => Ok(context.f32_type()),
        64 => Ok(context.f64_type()),
        _ => Err(Diagnostic::codegen_error(
            &format!("Invalid size for type : '{name}' at {size}"),
            SourceRange::undefined(),
        )),
    }
}

struct CastInstructionBuilder<'ctx, 'cnvn> {
    llvm: &'cnvn Llvm<'ctx>,
    index: &'cnvn Index,
    llvm_type_index: &'cnvn LlvmTypedIndex<'ctx>,
    value_type: &'cnvn DataTypeInformation,
    target_type: &'cnvn DataTypeInformation,
}

impl<'ctx, 'cast> CastInstructionBuilder<'ctx, 'cast> {
    fn generate_cast_instruction(
        llvm: &'cast Llvm<'ctx>,
        index: &'cast Index,
        llvm_type_index: &'cast LlvmTypedIndex<'ctx>,
        value: BasicValueEnum<'ctx>,
        value_type: &DataType,
        target_type: &DataType,
    ) -> Result<BasicValueEnum<'ctx>, Diagnostic> {
        let Some(cast_data) = CastInstructionBuilder::new(llvm, index, llvm_type_index, value_type, target_type) else {
                return Ok(value)
        };

        value.cast(cast_data)
    }

    fn new(
        llvm: &'cast Llvm<'ctx>,
        index: &'cast Index,
        llvm_type_index: &'cast LlvmTypedIndex<'ctx>,
        value_type: &DataType,
        target_type: &DataType,
    ) -> Option<CastInstructionBuilder<'ctx, 'cast>> {
        let target_type = index.get_intrinsic_type_by_name(target_type.get_name()).get_type_information();
        let value_type = index.get_intrinsic_type_by_name(value_type.get_name()).get_type_information();

        // if the current or target type are generic (unresolved or builtin)
        // we return the value without modification -> no cast info struct needed
        if target_type.is_generic(index) || value_type.is_generic(index) {
            return None;
        };

        let target_type =
            if let DataTypeInformation::Pointer { auto_deref: true, inner_type_name, .. } = target_type {
                // Deref auto-deref pointers before casting
                index.get_intrinsic_type_by_name(inner_type_name.as_str()).get_type_information()
            } else {
                target_type
            };

        Some(CastInstructionBuilder { llvm, index, llvm_type_index, value_type, target_type })
    }
}

trait Castable<'ctx, 'cast> {
    fn cast(self, cast_data: CastInstructionBuilder<'ctx, 'cast>)
        -> Result<BasicValueEnum<'ctx>, Diagnostic>;
}

trait Promotable<'ctx, 'cast> {
    fn promote(
        self,
        cast_data: CastInstructionBuilder<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, Diagnostic>;
}

trait Truncatable<'ctx, 'cast> {
    fn truncate(
        self,
        lsize: u32,
        cast_data: CastInstructionBuilder<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, Diagnostic>;
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for BasicValueEnum<'ctx> {
    fn cast(
        self,
        cast_data: CastInstructionBuilder<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, Diagnostic> {
        match self {
            BasicValueEnum::IntValue(val) => val.cast(cast_data),
            BasicValueEnum::FloatValue(val) => val.cast(cast_data),
            BasicValueEnum::PointerValue(val) => val.cast(cast_data),
            _ => Ok(self),
        }
    }
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for IntValue<'ctx> {
    fn cast(
        self,
        cast_data: CastInstructionBuilder<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, Diagnostic> {
        let lsize = cast_data.target_type.get_size_in_bits(cast_data.index);
        match cast_data.target_type {
            DataTypeInformation::Integer { .. } if cast_data.target_type.is_character() => {
                // special char assignment handling
                self.truncate(lsize, cast_data)
            }
            DataTypeInformation::Integer { .. } => {
                //its important to use the real type's size here, because we may have an i1 which is annotated as BOOL (8 bit)
                let rsize = self.get_type().get_bit_width();
                if lsize < rsize {
                    //Truncate
                    self.truncate(lsize, cast_data)
                } else {
                    //Expand
                    self.promote(cast_data)
                }
            }
            DataTypeInformation::Float { size: _rsize, .. } => {
                if cast_data.value_type.is_signed_int() {
                    Ok(cast_data
                        .llvm
                        .builder
                        .build_signed_int_to_float(
                            self,
                            get_llvm_float_type(cast_data.llvm.context, lsize, "Float")?,
                            "",
                        )
                        .into())
                } else {
                    Ok(cast_data
                        .llvm
                        .builder
                        .build_unsigned_int_to_float(
                            self,
                            get_llvm_float_type(cast_data.llvm.context, lsize, "Float")?,
                            "",
                        )
                        .into())
                }
            }
            DataTypeInformation::Pointer { .. } => Ok(cast_data
                .llvm
                .builder
                .build_int_to_ptr(
                    self,
                    cast_data
                        .llvm_type_index
                        .get_associated_type(cast_data.target_type.get_name())?
                        .into_pointer_type(),
                    "",
                )
                .into()),
            _ => unreachable!("Cannot cast integer value to {}", cast_data.target_type.get_name()),
        }
    }
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for FloatValue<'ctx> {
    fn cast(
        self,
        cast_data: CastInstructionBuilder<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, Diagnostic> {
        let rsize = &cast_data.value_type.get_size_in_bits(cast_data.index);
        match cast_data.target_type {
            DataTypeInformation::Float { size: lsize, .. } => {
                if lsize < rsize {
                    self.truncate(*lsize, cast_data)
                } else {
                    self.promote(cast_data)
                }
            }
            DataTypeInformation::Integer { signed, size: lsize, .. } => {
                if *signed {
                    Ok(cast_data
                        .llvm
                        .builder
                        .build_float_to_signed_int(
                            self,
                            get_llvm_int_type(cast_data.llvm.context, *lsize, "Integer")?,
                            "",
                        )
                        .into())
                } else {
                    Ok(cast_data
                        .llvm
                        .builder
                        .build_float_to_unsigned_int(
                            self,
                            get_llvm_int_type(cast_data.llvm.context, *lsize, "Integer")?,
                            "",
                        )
                        .into())
                }
            }
            _ => unreachable!("Cannot cast floating-point value to {}", cast_data.target_type.get_name()),
        }
    }
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for PointerValue<'ctx> {
    fn cast(
        self,
        cast_data: CastInstructionBuilder<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, Diagnostic> {
        let builder = &cast_data.llvm.builder;
        let llvm_type_index = cast_data.llvm_type_index;
        match &cast_data.target_type {
            DataTypeInformation::Integer { size: lsize, .. } => Ok(builder
                .build_ptr_to_int(self, get_llvm_int_type(cast_data.llvm.context, *lsize, "")?, "")
                .into()),
            DataTypeInformation::Pointer { .. } | DataTypeInformation::Void { .. } => {
                // TODO: is void really needed here? no failing tests if omitted/do we ever cast to void?
                let target_ptr_type =
                    llvm_type_index.get_associated_type(cast_data.target_type.get_name())?;
                if BasicValueEnum::from(self).get_type() != target_ptr_type {
                    // bit-cast necessary
                    Ok(builder.build_bitcast(self, target_ptr_type, ""))
                } else {
                    //this is ok, no cast required
                    Ok(self.into())
                }
            }
            _ => unreachable!("Cannot cast pointer value to {}", cast_data.target_type.get_name()),
        }
    }
}

impl<'ctx, 'cast> Promotable<'ctx, 'cast> for IntValue<'ctx> {
    fn promote(
        self,
        cast_data: CastInstructionBuilder<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, Diagnostic> {
        match &cast_data.target_type {
            DataTypeInformation::Integer { size: target_size, .. } => {
                // INT --> INT
                if &self.get_type().get_bit_width() < target_size {
                    let llvm_int_type = get_llvm_int_type(cast_data.llvm.context, *target_size, "Integer")?;
                    Ok(if cast_data.value_type.is_signed_int() {
                        cast_data.llvm.builder.build_int_s_extend_or_bit_cast(self, llvm_int_type, "")
                    } else {
                        cast_data.llvm.builder.build_int_z_extend_or_bit_cast(self, llvm_int_type, "")
                    }
                    .into())
                } else {
                    Ok(self.into())
                }
            }
            DataTypeInformation::Float { size: target_size, .. } => {
                // INT --> FLOAT
                let llvm_fp_type = get_llvm_float_type(cast_data.llvm.context, *target_size, "Float")?;
                Ok(if cast_data.value_type.is_signed_int() {
                    cast_data.llvm.builder.build_signed_int_to_float(self, llvm_fp_type, "")
                } else {
                    cast_data.llvm.builder.build_unsigned_int_to_float(self, llvm_fp_type, "")
                }
                .into())
            }
            _ => unreachable!("Can only promote to either INT or FLOAT types"),
        }
    }
}

impl<'ctx, 'cast> Promotable<'ctx, 'cast> for FloatValue<'ctx> {
    fn promote(
        self,
        cast_data: CastInstructionBuilder<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, Diagnostic> {
        // FLOAT --> FLOAT
        let target_size = cast_data.target_type.get_size_in_bits(cast_data.index);
        let value_size = cast_data.value_type.get_size_in_bits(cast_data.index);
        if target_size <= value_size {
            Ok(self.into())
        } else {
            Ok(cast_data
                .llvm
                .builder
                .build_float_ext(self, get_llvm_float_type(cast_data.llvm.context, target_size, "Float")?, "")
                .into())
        }
    }
}

impl<'ctx, 'cast> Truncatable<'ctx, 'cast> for IntValue<'ctx> {
    fn truncate(
        self,
        lsize: u32,
        cast_data: CastInstructionBuilder<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, Diagnostic> {
        Ok(cast_data
            .llvm
            .builder
            .build_int_truncate_or_bit_cast(
                self,
                get_llvm_int_type(cast_data.llvm.context, lsize, "Integer")?,
                "",
            )
            .into())
    }
}

impl<'ctx, 'cast> Truncatable<'ctx, 'cast> for FloatValue<'ctx> {
    fn truncate(
        self,
        lsize: u32,
        cast_data: CastInstructionBuilder<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, Diagnostic> {
        Ok(cast_data
            .llvm
            .builder
            .build_float_trunc(self, get_llvm_float_type(cast_data.llvm.context, lsize, "Float")?, "")
            .into())
    }
}
