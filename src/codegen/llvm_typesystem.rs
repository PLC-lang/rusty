// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use inkwell::{
    context::Context,
    types::{FloatType, IntType},
    values::{ArrayValue, BasicValueEnum, FloatValue, IntValue, PointerValue},
};

use crate::{
    index::Index,
    resolver::StatementAnnotation,
    typesystem::{DataType, DataTypeInformation, InternalType, StructSource},
};

use super::{
    generators::{
        expression_generator::ExpressionCodeGenerator, llvm::Llvm,
        statement_generator::StatementCodeGenerator,
    },
    llvm_index::LlvmTypedIndex,
};

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

pub trait TypeCaster<'ctx> {
    /// Generates a cast from the given `value` to the given `target_type` if necessary and returns the casted value. It returns
    /// the original `value` if no cast is necessary or if the provided value is not eligible to be cast (to the target type or at all).
    ///
    /// This function provides no additional validation or safeguards for invalid casts, as such validation is expected to be
    /// performed at the validation stage prior to code-gen.
    /// Cast instructions for values other than IntValue, FloatValue and PointerValue will simply be ignored (and the value
    /// returned unchanged). Invalid casting instructions for the above-mentioned values will fail spectacularly instead.
    ///
    /// - `self` the generator calling this function
    /// - `target_type` the expected target type of the value
    /// - `value_type` the current type of the given value
    /// - `value` the value to (maybe) cast
    fn cast_if_needed(
        &self,
        target_type: &DataType,
        value_type: &DataType,
        value: BasicValueEnum<'ctx>,
        annotation: Option<&StatementAnnotation>,
    ) -> BasicValueEnum<'ctx>;
}

/// Implementing this trait allows borrowing fields from structs when they are passed as generic arguments
pub trait Lender<'ctx, 'cast> {
    type Output;
    fn borrow(&self) -> Self::Output;
}

macro_rules! impl_borrow_for_generator {
    (($out1:ty, $out2:ty, $out3:ty), [$($t:ty),+]) => {
        $(impl<'ctx, 'cast> Lender<'ctx, 'cast> for $t {
            type Output = (&'cast $out1, &'cast $out2, &'cast $out3);
            fn borrow(&self) -> Self::Output {
                (self.index, self.llvm, self.llvm_index)
            }
        })*
    }
}

impl_borrow_for_generator! {(Index, Llvm<'ctx>, LlvmTypedIndex<'ctx>), [ExpressionCodeGenerator<'ctx, 'cast>, StatementCodeGenerator<'ctx, 'cast>]}

impl<'ctx, 'cast> TypeCaster<'ctx> for ExpressionCodeGenerator<'ctx, 'cast> {
    fn cast_if_needed(
        &self,
        target_type: &DataType,
        value_type: &DataType,
        value: BasicValueEnum<'ctx>,
        annotation: Option<&StatementAnnotation>,
    ) -> BasicValueEnum<'ctx> {
        value.cast(&CastInstructionGenerator::new(self, value_type, target_type, annotation))
    }
}

impl<'ctx, 'cast> TypeCaster<'ctx> for StatementCodeGenerator<'ctx, 'cast> {
    fn cast_if_needed(
        &self,
        target_type: &DataType,
        value_type: &DataType,
        value: BasicValueEnum<'ctx>,
        annotation: Option<&StatementAnnotation>,
    ) -> BasicValueEnum<'ctx> {
        value.cast(&CastInstructionGenerator::new(self, value_type, target_type, annotation))
    }
}

struct CastInstructionGenerator<'ctx, 'cast> {
    llvm: &'cast Llvm<'ctx>,
    index: &'cast Index,
    llvm_type_index: &'cast LlvmTypedIndex<'ctx>,
    value_type: &'cast DataTypeInformation,
    target_type: &'cast DataTypeInformation,
    annotation: Option<&'cast StatementAnnotation>,
}

impl<'ctx, 'cast> CastInstructionGenerator<'ctx, 'cast> {
    fn new<G>(
        generator: &G,
        value_type: &DataType,
        target_type: &DataType,
        annotation: Option<&'cast StatementAnnotation>,
    ) -> CastInstructionGenerator<'ctx, 'cast>
    where
        G: Lender<'ctx, 'cast, Output = (&'cast Index, &'cast Llvm<'ctx>, &'cast LlvmTypedIndex<'ctx>)>
            + TypeCaster<'ctx>,
    {
        let (index, llvm, llvm_type_index) = generator.borrow();
        let target_type = index.get_intrinsic_type_by_name(target_type.get_name()).get_type_information();
        let value_type = index.get_intrinsic_type_by_name(value_type.get_name()).get_type_information();

        let target_type =
            if let DataTypeInformation::Pointer { auto_deref: true, inner_type_name, .. } = target_type {
                // Deref auto-deref pointers before casting
                index.get_intrinsic_type_by_name(inner_type_name.as_str()).get_type_information()
            } else {
                target_type
            };
        CastInstructionGenerator { llvm, index, llvm_type_index, value_type, target_type, annotation }
    }
}

trait Castable<'ctx, 'cast> {
    fn cast(self, generator: &CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx>;
}

trait Promotable<'ctx, 'cast> {
    fn promote(self, lsize: u32, generator: &CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx>;
}

trait Truncatable<'ctx, 'cast> {
    fn truncate(self, lsize: u32, generator: &CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx>;
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for BasicValueEnum<'ctx> {
    fn cast(self, generator: &CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        match self {
            BasicValueEnum::IntValue(val) => val.cast(generator),
            BasicValueEnum::FloatValue(val) => val.cast(generator),
            BasicValueEnum::PointerValue(val) => val.cast(generator),
            BasicValueEnum::ArrayValue(val) => val.cast(generator),
            _ => self,
        }
    }
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for IntValue<'ctx> {
    fn cast(self, generatr: &CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        let lsize = generatr.target_type.get_size_in_bits(generatr.index);
        match generatr.target_type {
            DataTypeInformation::Integer { .. } => {
                //its important to use the real type's size here, because we may have an i1 which is annotated as BOOL (8 bit)
                let rsize = self.get_type().get_bit_width();
                if lsize < rsize {
                    //Truncate
                    self.truncate(lsize, generatr)
                } else {
                    //Expand
                    self.promote(lsize, generatr)
                }
            }
            DataTypeInformation::Float { .. } => {
                let float_type = get_llvm_float_type(generatr.llvm.context, lsize, "Float");
                if generatr.value_type.is_signed_int() {
                    generatr.llvm.builder.build_signed_int_to_float(self, float_type, "").into()
                } else {
                    generatr.llvm.builder.build_unsigned_int_to_float(self, float_type, "").into()
                }
            }
            DataTypeInformation::Pointer { .. } => {
                let Ok(associated_type) = generatr
                    .llvm_type_index
                    .get_associated_type(generatr.target_type.get_name()) else {
                        unreachable!("Target type of cast instruction does not exist: {}", generatr.target_type.get_name())
                    };

                generatr.llvm.builder.build_int_to_ptr(self, associated_type.into_pointer_type(), "").into()
            }
            _ => unreachable!("Cannot cast integer value to {}", generatr.target_type.get_name()),
        }
    }
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for FloatValue<'ctx> {
    fn cast(self, generator: &CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        let rsize = &generator.value_type.get_size_in_bits(generator.index);
        match generator.target_type {
            DataTypeInformation::Float { size: lsize, .. } => {
                if lsize < rsize {
                    self.truncate(*lsize, generator)
                } else {
                    self.promote(*lsize, generator)
                }
            }
            DataTypeInformation::Integer { signed, size: lsize, .. } => {
                let int_type = get_llvm_int_type(generator.llvm.context, *lsize, "Integer");
                if *signed {
                    generator.llvm.builder.build_float_to_signed_int(self, int_type, "").into()
                } else {
                    generator.llvm.builder.build_float_to_unsigned_int(self, int_type, "").into()
                }
            }
            _ => unreachable!("Cannot cast floating-point value to {}", generator.target_type.get_name()),
        }
    }
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for PointerValue<'ctx> {
    fn cast(self, generator: &CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        match &generator.target_type {
            DataTypeInformation::Integer { size: lsize, .. } => generator
                .llvm
                .builder
                .build_ptr_to_int(self, get_llvm_int_type(generator.llvm.context, *lsize, ""), "")
                .into(),
            DataTypeInformation::Pointer { .. } => {
                let Ok(target_ptr_type) = generator.llvm_type_index.get_associated_type(generator.target_type.get_name()) else {
                        unreachable!("Target type of cast instruction does not exist: {}", generator.target_type.get_name())
                    };
                if BasicValueEnum::from(self).get_type() != target_ptr_type {
                    // bit-cast necessary
                    generator.llvm.builder.build_bitcast(self, target_ptr_type, "")
                } else {
                    //this is ok, no cast required
                    self.into()
                }
            }
            DataTypeInformation::Struct {
                source: StructSource::Internal(InternalType::VariableLengthArray { .. }),
                ..
            } => {
                // we are dealing with an auto-deref vla parameter. first we have to deref our array and build the fat pointer
                let struct_val = generator.llvm.builder.build_load(self, "auto_deref").cast(generator);

                // create a pointer to the generated StructValue
                let struct_ptr = generator.llvm.builder.build_alloca(struct_val.get_type(), "vla_struct_ptr");
                generator.llvm.builder.build_store(struct_ptr, struct_val);
                struct_ptr.into()
            }
            _ => unreachable!("Cannot cast pointer value to {}", generator.target_type.get_name()),
        }
    }
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for ArrayValue<'ctx> {
    /// Generates a fat pointer struct for an array if the target type is a VLA,
    /// otherwise returns the value as is.
    fn cast(self, generator: &CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        if !generator.target_type.is_vla() {
            return self.into();
        }
        let builder = &generator.llvm.builder;
        let zero = generator.llvm.i32_type().const_zero();

        let Ok(associated_type) = generator
            .llvm_type_index
            .get_associated_type(generator.target_type.get_name()) else {
                unreachable!("Target type of cast instruction does not exist: {}", generator.target_type.get_name())
        };

        // Get array annotation from parent POU and get pointer to array
        let Some(StatementAnnotation::Variable { qualified_name, .. }) = generator.annotation  else {
            unreachable!("Undefined reference: {}", generator.value_type.get_name())
        };
        let array_pointer = generator
            .llvm_type_index
            .find_loaded_associated_variable_value(qualified_name.as_str())
            .unwrap_or_else(|| unreachable!("passed array must be in the llvm index"));

        // gep into the original array. the resulting address will be stored in the VLA struct
        let arr_gep = unsafe { builder.build_in_bounds_gep(array_pointer, &[zero, zero], "outer_arr_gep") };

        // -- Generate struct & arr_ptr --
        let ty = associated_type.into_struct_type();
        let vla_struct = builder.build_alloca(ty, "vla_struct");

        let Ok(vla_arr_ptr) = builder.build_struct_gep(vla_struct, 0, "vla_array_gep") else {
            unreachable!("Must have a valid, GEP-able fat-pointer struct at this stage")
        };

        let Ok(vla_dimensions_ptr) = builder.build_struct_gep(vla_struct, 1, "vla_dimensions_gep") else {
            unreachable!("Must have a valid, GEP-able fat-pointer struct at this stage")
        };

        // -- Generate dimensions --
        let DataTypeInformation::Array { dimensions, .. } = generator.value_type else { unreachable!() };
        let mut dims = Vec::new();
        for dim in dimensions {
            dims.push(dim.start_offset.as_int_value(generator.index).unwrap());
            dims.push(dim.end_offset.as_int_value(generator.index).unwrap());
        }

        // Populate each array element
        let dimensions =
            dims.iter().map(|it| generator.llvm.i32_type().const_int(*it as u64, true)).collect::<Vec<_>>();
        let array_value = generator.llvm.i32_type().const_array(&dimensions);
        builder.build_store(vla_dimensions_ptr, array_value);

        builder.build_store(vla_arr_ptr, arr_gep);

        builder.build_load(vla_struct, "")
    }
}

impl<'ctx, 'cast> Promotable<'ctx, 'cast> for IntValue<'ctx> {
    fn promote(self, lsize: u32, generatr: &CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        let llvm_int_type = get_llvm_int_type(generatr.llvm.context, lsize, "Integer");
        if generatr.value_type.is_signed_int() {
            generatr.llvm.builder.build_int_s_extend_or_bit_cast(self, llvm_int_type, "")
        } else {
            generatr.llvm.builder.build_int_z_extend_or_bit_cast(self, llvm_int_type, "")
        }
        .into()
    }
}

impl<'ctx, 'cast> Promotable<'ctx, 'cast> for FloatValue<'ctx> {
    fn promote(self, lsize: u32, generator: &CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        generator
            .llvm
            .builder
            .build_float_ext(self, get_llvm_float_type(generator.llvm.context, lsize, "Float"), "")
            .into()
    }
}

impl<'ctx, 'cast> Truncatable<'ctx, 'cast> for IntValue<'ctx> {
    fn truncate(self, lsize: u32, generator: &CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        generator
            .llvm
            .builder
            .build_int_truncate_or_bit_cast(
                self,
                get_llvm_int_type(generator.llvm.context, lsize, "Integer"),
                "",
            )
            .into()
    }
}

impl<'ctx, 'cast> Truncatable<'ctx, 'cast> for FloatValue<'ctx> {
    fn truncate(self, lsize: u32, generator: &CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        generator
            .llvm
            .builder
            .build_float_trunc(self, get_llvm_float_type(generator.llvm.context, lsize, "Float"), "")
            .into()
    }
}
