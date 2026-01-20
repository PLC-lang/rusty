// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use inkwell::{
    context::Context,
    types::{FloatType, IntType},
    values::{ArrayValue, BasicValueEnum, FloatValue, IntValue, PointerValue},
};
use plc_source::source_location::SourceLocation;

use crate::{
    index::Index,
    resolver::StatementAnnotation,
    typesystem::{DataType, DataTypeInformation, InternalType, StructSource},
};

use super::{generators::llvm::Llvm, llvm_index::LlvmTypedIndex, CodegenError};

/// A convenience macro to call the `cast` function with fewer parameters.
///
/// Generates a cast from the given `value` to the given `target_type` if necessary and returns the casted value. It returns
/// the original `value` if no cast is necessary or if the provided value is not eligible to be cast (to the target type or at all).
///
/// This function provides no additional validation or safeguards for invalid casts, as such validation is expected to be
/// performed at the validation stage prior to code-gen.
/// Cast instructions for values other than IntValue, FloatValue and PointerValue will simply be ignored (and the value
/// returned unchanged). Invalid casting instructions for the above-mentioned values will fail spectacularly instead.
///
/// - `generator` the generator to use. must contain the following fields:
///     - `llvm` the llvm utilities to use for code-generation
///     - `index` the current Index used for type-lookups
///     - `llvm_type_index` the type index to lookup llvm generated types
/// - `target_type` the expected target type of the value
/// - `value_type` the current type of the given value
/// - `value` the value to (maybe) cast
macro_rules! cast_if_needed {
    ($generator:expr, $target_type:expr, $value_type:expr, $value:expr, $annotation:expr) => {
        crate::codegen::llvm_typesystem::cast(
            $generator.llvm,
            $generator.index,
            $generator.llvm_index,
            $target_type,
            $value_type,
            $value,
            $annotation,
        )
    };
}

pub(crate) use cast_if_needed;

pub fn get_llvm_int_type<'a>(context: &'a Context, size: u32, name: &str) -> IntType<'a> {
    match size {
        1 => context.bool_type(),
        8 => context.i8_type(),
        16 => context.i16_type(),
        32 => context.i32_type(),
        64 => context.i64_type(),
        128 => context.i128_type(),
        _ => unreachable!("Invalid size for type : '{name}' at {size}"),
    }
}

pub fn get_llvm_float_type<'a>(context: &'a Context, size: u32, name: &str) -> FloatType<'a> {
    match size {
        32 => context.f32_type(),
        64 => context.f64_type(),
        _ => unreachable!("Invalid size for type : '{name}' at {size}"),
    }
}

/// Generates a cast from the given `value` to the given `target_type` if necessary and returns the casted value. It returns
/// the original `value` if no cast is necessary or if the provided value is not eligible to be cast (to the target type or at all).
///
/// This function provides no additional validation or safeguards for invalid casts, as such validation is expected to be
/// performed at the validation stage prior to code-gen.
/// Cast instructions for values other than IntValue, FloatValue and PointerValue will simply be ignored (and the value
/// returned unchanged). Invalid casting instructions for the above-mentioned values will fail spectacularly instead.
///
/// - `llvm` the llvm utilities to use for code-generation
/// - `index` the current Index used for type-lookups
/// - `llvm_type_index` the type index to lookup llvm generated types
/// - `target_type` the expected target type of the value
/// - `value_type` the current type of the given value
/// - `value` the value to (maybe) cast
pub fn cast<'ctx>(
    llvm: &Llvm<'ctx>,
    index: &Index,
    llvm_type_index: &LlvmTypedIndex<'ctx>,
    target_type: &DataType,
    value_type: &DataType,
    value: BasicValueEnum<'ctx>,
    annotation: Option<&StatementAnnotation>,
) -> Result<BasicValueEnum<'ctx>, CodegenError> {
    value.cast(&CastInstructionData::new(llvm, index, llvm_type_index, value_type, target_type, annotation))
}

struct CastInstructionData<'ctx, 'cast> {
    llvm: &'cast Llvm<'ctx>,
    index: &'cast Index,
    llvm_type_index: &'cast LlvmTypedIndex<'ctx>,
    value_type: &'cast DataTypeInformation,
    target_type: &'cast DataTypeInformation,
    annotation: Option<&'cast StatementAnnotation>,
}

impl<'ctx, 'cast> CastInstructionData<'ctx, 'cast> {
    fn new(
        llvm: &'cast Llvm<'ctx>,
        index: &'cast Index,
        llvm_type_index: &'cast LlvmTypedIndex<'ctx>,
        value_type: &DataType,
        target_type: &DataType,
        annotation: Option<&'cast StatementAnnotation>,
    ) -> Self {
        let target_type = index.get_intrinsic_type_by_name(target_type.get_name()).get_type_information();
        let value_type = index.get_intrinsic_type_by_name(value_type.get_name()).get_type_information();

        let target_type =
            if let DataTypeInformation::Pointer { auto_deref: Some(_), inner_type_name, .. } = target_type {
                // Deref auto-deref pointers before casting
                index.get_intrinsic_type_by_name(inner_type_name.as_str()).get_type_information()
            } else {
                target_type
            };

        CastInstructionData { llvm, index, llvm_type_index, value_type, target_type, annotation }
    }
}

trait Castable<'ctx, 'cast> {
    fn cast(self, cast_data: &CastInstructionData<'ctx, 'cast>)
        -> Result<BasicValueEnum<'ctx>, CodegenError>;
    /// Casts a constant value. This is a separate function to allow for optimizations when dealing
    /// with constant values.
    fn cast_constant(
        self,
        cast_data: &CastInstructionData<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError>;
}

trait Promotable<'ctx, 'cast> {
    fn promote(
        self,
        lsize: u32,
        cast_data: &CastInstructionData<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError>;
}

trait Truncatable<'ctx, 'cast> {
    fn truncate(
        self,
        lsize: u32,
        cast_data: &CastInstructionData<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError>;
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for BasicValueEnum<'ctx> {
    fn cast(
        self,
        cast_data: &CastInstructionData<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        // If the builder is not yet set, ignore casting
        if cast_data.llvm.builder.get_insert_block().is_none() {
            log::debug!("The builder is not set, attempting a constant cast");
            if let Ok(const_val) = self.cast_constant(cast_data) {
                return Ok(const_val);
            }
            log::debug!("Constant cast failed, returning original value");
            return Ok(self);
        }
        Ok(match self {
            BasicValueEnum::IntValue(val) => val.cast(cast_data)?,
            BasicValueEnum::FloatValue(val) => val.cast(cast_data)?,
            BasicValueEnum::PointerValue(val) => val.cast(cast_data)?,
            BasicValueEnum::ArrayValue(val) => val.cast(cast_data)?,
            _ => self,
        })
    }

    fn cast_constant(
        self,
        cast_data: &CastInstructionData<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        Ok(match self {
            BasicValueEnum::IntValue(val) => val.cast_constant(cast_data)?,
            BasicValueEnum::FloatValue(val) => val.cast_constant(cast_data)?,
            // Pointers and Arrays cannot be constants in our use-case, so just call the normal
            // cast
            BasicValueEnum::PointerValue(val) => val.cast(cast_data)?,
            BasicValueEnum::ArrayValue(val) => val.cast(cast_data)?,
            _ => self,
        })
    }
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for IntValue<'ctx> {
    fn cast(
        self,
        cast_data: &CastInstructionData<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        let lsize = cast_data
            .target_type
            .get_size_in_bits(cast_data.index)
            .map_err(|it| CodegenError::new(it.to_string(), SourceLocation::undefined()))?;
        match cast_data.target_type {
            DataTypeInformation::Integer { .. } => {
                //its important to use the real type's size here, because we may have an i1 which is annotated as BOOL (8 bit)
                let rsize = self.get_type().get_bit_width();
                if lsize < rsize {
                    //Truncate
                    self.truncate(lsize, cast_data)
                } else if lsize > rsize {
                    //Expand
                    self.promote(lsize, cast_data)
                } else {
                    //same size, no cast needed
                    Ok(self.into())
                }
            }
            DataTypeInformation::Float { .. } => {
                let float_type = get_llvm_float_type(cast_data.llvm.context, lsize, "Float");
                if cast_data.value_type.is_signed_int() {
                    cast_data
                        .llvm
                        .builder
                        .build_signed_int_to_float(self, float_type, "")
                        .map(Into::into)
                        .map_err(Into::into)
                } else {
                    cast_data
                        .llvm
                        .builder
                        .build_unsigned_int_to_float(self, float_type, "")
                        .map(Into::into)
                        .map_err(Into::into)
                }
            }
            DataTypeInformation::Pointer { .. } => {
                let associated_type =
                    cast_data.llvm_type_index.get_associated_type(cast_data.target_type.get_name())?;
                cast_data
                    .llvm
                    .builder
                    .build_int_to_ptr(self, associated_type.into_pointer_type(), "")
                    .map(Into::into)
                    .map_err(Into::into)
            }
            _ => Err(CodegenError::new(
                format!("Cannot cast integer value to {}", cast_data.target_type.get_name()),
                SourceLocation::undefined(),
            )),
        }
    }

    fn cast_constant(
        self,
        cast_data: &CastInstructionData<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        // If self is not a constant, fail
        if !self.is_constant_int() {
            return Err(CodegenError::new(
                "Value is not a constant".to_string(),
                SourceLocation::undefined(),
            ));
        }

        let mut signed_value = None;
        let mut unsigned_value = None;
        if cast_data.value_type.is_signed_int() {
            signed_value = self.get_sign_extended_constant()
        } else {
            unsigned_value = self.get_zero_extended_constant()
        };

        match cast_data.target_type {
            DataTypeInformation::Integer { .. } => {
                let target_type = cast_data
                    .llvm_type_index
                    .get_associated_type(cast_data.target_type.get_name())?
                    .into_int_type();
                if let Some(sval) = signed_value {
                    Ok(target_type.const_int(sval as u64, true).into())
                } else if let Some(uval) = unsigned_value {
                    Ok(target_type.const_int(uval, false).into())
                } else {
                    Err(CodegenError::new("Value is not a constant".to_string(), SourceLocation::undefined()))
                }
            }
            DataTypeInformation::Float { size: lsize, .. } => {
                let float_type = get_llvm_float_type(cast_data.llvm.context, *lsize, "Float");
                if let Some(sval) = signed_value {
                    Ok(float_type.const_float(sval as f64).into())
                } else if let Some(uval) = unsigned_value {
                    Ok(float_type.const_float(uval as f64).into())
                } else {
                    Err(CodegenError::new("Value is not a constant".to_string(), SourceLocation::undefined()))
                }
            }
            _ => Err(CodegenError::new(
                format!("Cannot cast integer value to a constant {}", cast_data.target_type.get_name()),
                SourceLocation::undefined(),
            )),
        }
    }
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for FloatValue<'ctx> {
    fn cast(
        self,
        cast_data: &CastInstructionData<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        let rsize = &cast_data.value_type.get_size_in_bits(cast_data.index).unwrap();
        match cast_data.target_type {
            DataTypeInformation::Float { size: lsize, .. } => {
                if lsize < rsize {
                    self.truncate(*lsize, cast_data)
                } else if lsize > rsize {
                    self.promote(*lsize, cast_data)
                } else {
                    //same size, no cast needed
                    Ok(self.into())
                }
            }
            DataTypeInformation::Integer { signed, size: lsize, .. } => {
                let int_type = get_llvm_int_type(cast_data.llvm.context, *lsize, "Integer");
                if *signed {
                    cast_data
                        .llvm
                        .builder
                        .build_float_to_signed_int(self, int_type, "")
                        .map(Into::into)
                        .map_err(Into::into)
                } else {
                    cast_data
                        .llvm
                        .builder
                        .build_float_to_unsigned_int(self, int_type, "")
                        .map(Into::into)
                        .map_err(Into::into)
                }
            }
            _ => Err(CodegenError::new(
                format!("Cannot cast float value to {}", cast_data.target_type.get_name()),
                SourceLocation::undefined(),
            )),
        }
    }

    fn cast_constant(
        self,
        cast_data: &CastInstructionData<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        // If self is not a constant, fail
        let Some((float_value, _)) = self.get_constant() else {
            return Err(CodegenError::new(
                "Value is not a constant".to_string(),
                SourceLocation::undefined(),
            ));
        };

        match cast_data.target_type {
            DataTypeInformation::Float { size: lsize, .. } => {
                let target_type = get_llvm_float_type(cast_data.llvm.context, *lsize, "Float");
                Ok(target_type.const_float(float_value).into())
            }
            DataTypeInformation::Integer { signed, size: lsize, .. } => {
                let int_type = get_llvm_int_type(cast_data.llvm.context, *lsize, "Integer");
                if *signed {
                    Ok(int_type.const_int(float_value as i64 as u64, true).into())
                } else {
                    Ok(int_type.const_int(float_value as u64, false).into())
                }
            }
            _ => Err(CodegenError::new(
                format!("Cannot cast float value to a constant {}", cast_data.target_type.get_name()),
                SourceLocation::undefined(),
            )),
        }
    }
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for PointerValue<'ctx> {
    fn cast(
        self,
        cast_data: &CastInstructionData<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        match &cast_data.target_type {
            DataTypeInformation::Integer { size: lsize, .. } => Ok(cast_data
                .llvm
                .builder
                .build_ptr_to_int(self, get_llvm_int_type(cast_data.llvm.context, *lsize, ""), "")?
                .into()),
            DataTypeInformation::Pointer { .. } => {
                let target_ptr_type =
                    cast_data.llvm_type_index.get_associated_type(cast_data.target_type.get_name())?;
                if BasicValueEnum::from(self).get_type() != target_ptr_type {
                    // bit-cast necessary
                    cast_data.llvm.builder.build_bit_cast(self, target_ptr_type, "").map_err(Into::into)
                } else {
                    //this is ok, no cast required
                    Ok(self.into())
                }
            }
            DataTypeInformation::Struct {
                source: StructSource::Internal(InternalType::VariableLengthArray { .. }),
                ..
            } => {
                // we are dealing with an auto-deref vla parameter. first we have to deref our array and build the fat pointer
                let pointee =
                    cast_data.llvm_type_index.get_associated_type(cast_data.value_type.get_name())?;
                let struct_val =
                    cast_data.llvm.builder.build_load(pointee, self, "auto_deref")?.cast(cast_data)?;

                // create a pointer to the generated StructValue
                let struct_ptr =
                    cast_data.llvm.builder.build_alloca(struct_val.get_type(), "vla_struct_ptr")?;
                cast_data.llvm.builder.build_store(struct_ptr, struct_val)?;
                Ok(struct_ptr.into())
            }
            _ => Err(CodegenError::new(
                format!("Cannot cast pointer value to {}", cast_data.target_type.get_name()),
                SourceLocation::undefined(),
            )),
        }
    }

    fn cast_constant(
        self,
        cast_data: &CastInstructionData<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        // Pointers cannot be constants in our use-case, so just call the normal cast
        self.cast(cast_data)
    }
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for ArrayValue<'ctx> {
    /// Generates a fat pointer struct for an array if the target type is a VLA,
    /// otherwise returns the value as is.
    fn cast(
        self,
        cast_data: &CastInstructionData<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        if !cast_data.target_type.is_vla() {
            return Ok(self.into());
        }
        let builder = &cast_data.llvm.builder;
        let zero = cast_data.llvm.i32_type().const_zero();

        let associated_type =
            cast_data.llvm_type_index.get_associated_type(cast_data.target_type.get_name())?;

        // Get array annotation from parent POU and get pointer to array
        let Some(StatementAnnotation::Variable { qualified_name, .. }) = cast_data.annotation else {
            return Err(CodegenError::new(
                format!("Undefined reference: {}", cast_data.value_type.get_name()),
                SourceLocation::undefined(),
            ));
        };
        let array_pointer = cast_data
            .llvm_type_index
            .find_loaded_associated_variable_value(qualified_name.as_str())
            .ok_or_else(|| {
                CodegenError::new(
                    format!("Cannot find array pointer for {}", cast_data.value_type.get_name()),
                    SourceLocation::undefined(),
                )
            })?;

        // gep into the original array. the resulting address will be stored in the VLA struct
        let pointee = cast_data.llvm_type_index.get_associated_type(cast_data.value_type.get_name())?;
        let arr_gep =
            unsafe { builder.build_in_bounds_gep(pointee, array_pointer, &[zero, zero], "outer_arr_gep")? };

        // -- Generate struct & arr_ptr --
        let ty = associated_type.into_struct_type();
        let vla_struct = builder.build_alloca(ty, "vla_struct")?;

        let vla_arr_ptr = builder.build_struct_gep(associated_type, vla_struct, 0, "vla_array_gep")?;

        let vla_dimensions_ptr =
            builder.build_struct_gep(associated_type, vla_struct, 1, "vla_dimensions_gep")?;

        // -- Generate dimensions --
        let DataTypeInformation::Array { dimensions, .. } = cast_data.value_type else { unreachable!() };
        let mut dims = Vec::new();
        for dim in dimensions {
            dims.push(dim.start_offset.as_int_value(cast_data.index).unwrap());
            dims.push(dim.end_offset.as_int_value(cast_data.index).unwrap());
        }

        // Populate each array element
        let dimensions =
            dims.iter().map(|it| cast_data.llvm.i32_type().const_int(*it as u64, true)).collect::<Vec<_>>();
        let array_value = cast_data.llvm.i32_type().const_array(&dimensions);
        // FIXME: should be memcopied, but is an rvalue. can only initialize global variables with value types. any other way for alloca'd variables than using store?
        builder.build_store(vla_dimensions_ptr, array_value)?;

        builder.build_store(vla_arr_ptr, arr_gep)?;

        builder.build_load(associated_type, vla_struct, "").map_err(Into::into)
    }

    fn cast_constant(
        self,
        cast_data: &CastInstructionData<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        // Arrays cannot be constants in our use-case, so just call the normal cast
        self.cast(cast_data)
    }
}

impl<'ctx, 'cast> Promotable<'ctx, 'cast> for IntValue<'ctx> {
    fn promote(
        self,
        lsize: u32,
        cast_data: &CastInstructionData<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        let llvm_int_type = get_llvm_int_type(cast_data.llvm.context, lsize, "Integer");
        Ok(if cast_data.value_type.is_signed_int() {
            cast_data.llvm.builder.build_int_s_extend_or_bit_cast(self, llvm_int_type, "")?
        } else {
            cast_data.llvm.builder.build_int_z_extend_or_bit_cast(self, llvm_int_type, "")?
        }
        .into())
    }
}

impl<'ctx, 'cast> Promotable<'ctx, 'cast> for FloatValue<'ctx> {
    fn promote(
        self,
        lsize: u32,
        cast_data: &CastInstructionData<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        Ok(cast_data
            .llvm
            .builder
            .build_float_ext(self, get_llvm_float_type(cast_data.llvm.context, lsize, "Float"), "")?
            .into())
    }
}

impl<'ctx, 'cast> Truncatable<'ctx, 'cast> for IntValue<'ctx> {
    fn truncate(
        self,
        lsize: u32,
        cast_data: &CastInstructionData<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        Ok(cast_data
            .llvm
            .builder
            .build_int_truncate_or_bit_cast(
                self,
                get_llvm_int_type(cast_data.llvm.context, lsize, "Integer"),
                "",
            )?
            .into())
    }
}

impl<'ctx, 'cast> Truncatable<'ctx, 'cast> for FloatValue<'ctx> {
    fn truncate(
        self,
        lsize: u32,
        cast_data: &CastInstructionData<'ctx, 'cast>,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        Ok(cast_data
            .llvm
            .builder
            .build_float_trunc(self, get_llvm_float_type(cast_data.llvm.context, lsize, "Float"), "")?
            .into())
    }
}
