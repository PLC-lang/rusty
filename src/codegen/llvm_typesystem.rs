// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use inkwell::{
    context::Context,
    types::{FloatType, IntType},
    values::{BasicValueEnum, FloatValue, IntValue, PointerValue},
};

use crate::{
    index::Index,
    typesystem::{DataType, DataTypeInformation},
};

use super::{
    generators::{
        expression_generator::ExpressionCodeGenerator, llvm::Llvm,
        statement_generator::StatementCodeGenerator,
    },
    llvm_index::LlvmTypedIndex,
};

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
pub trait CastMeMaybe<'ctx> {
    fn cast_if_needed(
        &self,
        target_type: &DataType,
        value_type: &DataType,
        value: BasicValueEnum<'ctx>,
    ) -> BasicValueEnum<'ctx>;
}

trait Generator<'ctx, 'cast> {
    type Output;
    fn borrow_ll(&self) -> Self::Output;
}

macro_rules! impl_generator {
    (($out1:ty, $out2:ty, $out3:ty), [$($t:ty),+]) => {
        $(impl<'ctx, 'cast> Generator<'ctx, 'cast> for $t {
            type Output = ($out1, $out2, $out3);
            fn borrow_ll(&self) -> Self::Output {
                (&self.index, &self.llvm, &self.llvm_index)
            }
        })*
    }
}

impl_generator! {(&'ctx Index, &'ctx Llvm<'ctx>, &'ctx LlvmTypedIndex<'ctx>), [ExpressionCodeGenerator<'ctx, 'cast>, StatementCodeGenerator<'ctx, 'cast>]}

impl<'ctx, 'cast> CastMeMaybe<'ctx> for ExpressionCodeGenerator<'ctx, 'cast> {
    fn cast_if_needed(
        &self,
        target_type: &DataType,
        value_type: &DataType,
        value: BasicValueEnum<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        value.cast(CastInstructionGenerator::new(self, value_type, target_type))
    }
}

// pub fn cast_if_needed<'ctx>(
//     llvm: &Llvm<'ctx>,
//     index: &Index,
//     llvm_type_index: &LlvmTypedIndex<'ctx>,
//     target_type: &DataType,
//     value_type: &DataType,
//     value: BasicValueEnum<'ctx>,
// ) -> BasicValueEnum<'ctx> {
//     value.cast(CastInstructionGenerator::new(llvm, index, llvm_type_index, value_type, target_type))
// }

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

// llvm: &'cast Llvm<'ctx>,
// index: &'cast Index,
// llvm_type_index: &'cast LlvmTypedIndex<'ctx>,

struct CastInstructionGenerator<'ctx, 'cast> {
    llvm: &'cast Llvm<'ctx>,
    index: &'cast Index,
    llvm_type_index: &'cast LlvmTypedIndex<'ctx>,
    value_type: &'cast DataTypeInformation,
    target_type: &'cast DataTypeInformation,
}

impl<'ctx, 'cast> CastInstructionGenerator<'ctx, 'cast> {
    fn new<G>(
        generator: G,
        value_type: &DataType,
        target_type: &DataType,
    ) -> CastInstructionGenerator<'ctx, 'cast>
    where
        G: Generator<'ctx, 'cast, Output = (&'ctx Index, &'ctx Llvm<'ctx>, &'ctx LlvmTypedIndex<'ctx>)>
            + CastMeMaybe<'ctx>,
    {
        let (index, llvm, llvm_type_index) = generator.borrow_ll();
        let target_type = index.get_intrinsic_type_by_name(target_type.get_name()).get_type_information();
        let value_type = index.get_intrinsic_type_by_name(value_type.get_name()).get_type_information();

        let target_type =
            if let DataTypeInformation::Pointer { auto_deref: true, inner_type_name, .. } = target_type {
                // Deref auto-deref pointers before casting
                index.get_intrinsic_type_by_name(inner_type_name.as_str()).get_type_information()
            } else {
                target_type
            };

        CastInstructionGenerator { llvm, index, llvm_type_index, value_type, target_type }
    }
}

trait Castable<'ctx, 'cast> {
    fn cast(self, generator: CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx>;
}

trait Promotable<'ctx, 'cast> {
    fn promote(self, lsize: u32, generator: CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx>;
}

trait Truncatable<'ctx, 'cast> {
    fn truncate(self, lsize: u32, generator: CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx>;
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for BasicValueEnum<'ctx> {
    fn cast(self, generator: CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        match self {
            BasicValueEnum::IntValue(val) => val.cast(generator),
            BasicValueEnum::FloatValue(val) => val.cast(generator),
            BasicValueEnum::PointerValue(val) => val.cast(generator),
            _ => self,
        }
    }
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for IntValue<'ctx> {
    fn cast(self, generatr: CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
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
    fn cast(self, generatr: CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        let rsize = &generatr.value_type.get_size_in_bits(generatr.index);
        match generatr.target_type {
            DataTypeInformation::Float { size: lsize, .. } => {
                if lsize < rsize {
                    self.truncate(*lsize, generatr)
                } else {
                    self.promote(*lsize, generatr)
                }
            }
            DataTypeInformation::Integer { signed, size: lsize, .. } => {
                let int_type = get_llvm_int_type(generatr.llvm.context, *lsize, "Integer");
                if *signed {
                    generatr.llvm.builder.build_float_to_signed_int(self, int_type, "").into()
                } else {
                    generatr.llvm.builder.build_float_to_unsigned_int(self, int_type, "").into()
                }
            }
            _ => unreachable!("Cannot cast floating-point value to {}", generatr.target_type.get_name()),
        }
    }
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for PointerValue<'ctx> {
    fn cast(self, generatr: CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        match &generatr.target_type {
            DataTypeInformation::Integer { size: lsize, .. } => generatr
                .llvm
                .builder
                .build_ptr_to_int(self, get_llvm_int_type(generatr.llvm.context, *lsize, ""), "")
                .into(),
            DataTypeInformation::Pointer { .. } | DataTypeInformation::Void { .. } => {
                // TODO: is void really needed here? no failing tests if omitted/do we ever cast to void?
                let Ok(target_ptr_type) = generatr.llvm_type_index.get_associated_type(generatr.target_type.get_name()) else {
                        unreachable!("Target type of cast instruction does not exist: {}", generatr.target_type.get_name())
                    };
                if BasicValueEnum::from(self).get_type() != target_ptr_type {
                    // bit-cast necessary
                    generatr.llvm.builder.build_bitcast(self, target_ptr_type, "")
                } else {
                    //this is ok, no cast required
                    self.into()
                }
            }
            _ => unreachable!("Cannot cast pointer value to {}", generatr.target_type.get_name()),
        }
    }
}

impl<'ctx, 'cast> Promotable<'ctx, 'cast> for IntValue<'ctx> {
    fn promote(self, lsize: u32, generatr: CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
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
    fn promote(self, lsize: u32, generator: CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        generator
            .llvm
            .builder
            .build_float_ext(self, get_llvm_float_type(generator.llvm.context, lsize, "Float"), "")
            .into()
    }
}

impl<'ctx, 'cast> Truncatable<'ctx, 'cast> for IntValue<'ctx> {
    fn truncate(self, lsize: u32, generator: CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
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
    fn truncate(self, lsize: u32, generatr: CastInstructionGenerator<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        generatr
            .llvm
            .builder
            .build_float_trunc(self, get_llvm_float_type(generatr.llvm.context, lsize, "Float"), "")
            .into()
    }
}
