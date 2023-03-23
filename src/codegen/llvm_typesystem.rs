// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use inkwell::{
    context::Context,
    types::{BasicType, FloatType, IntType},
    values::{ArrayValue, BasicValueEnum, FloatValue, IntValue, PointerValue},
    AddressSpace,
};

use crate::{
    index::Index,
    typesystem::{DataType, DataTypeInformation},
};

use super::{
    generators::{llvm::Llvm, ADDRESS_SPACE_GENERIC},
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
pub fn cast_if_needed<'ctx>(
    llvm: &Llvm<'ctx>,
    index: &Index,
    llvm_type_index: &LlvmTypedIndex<'ctx>,
    target_type: &DataType,
    value_type: &DataType,
    value: BasicValueEnum<'ctx>,
) -> BasicValueEnum<'ctx> {
    value.cast(CastInstructionData::new(llvm, index, llvm_type_index, value_type, target_type))
}

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

struct CastInstructionData<'ctx, 'cast> {
    llvm: &'cast Llvm<'ctx>,
    index: &'cast Index,
    llvm_type_index: &'cast LlvmTypedIndex<'ctx>,
    value_type: &'cast DataTypeInformation,
    target_type: &'cast DataTypeInformation,
}

impl<'ctx, 'cast> CastInstructionData<'ctx, 'cast> {
    fn new(
        llvm: &'cast Llvm<'ctx>,
        index: &'cast Index,
        llvm_type_index: &'cast LlvmTypedIndex<'ctx>,
        value_type: &DataType,
        target_type: &DataType,
    ) -> Self {
        let target_type = index.get_intrinsic_type_by_name(target_type.get_name()).get_type_information();
        let value_type = index.get_intrinsic_type_by_name(value_type.get_name()).get_type_information();

        let target_type =
            if let DataTypeInformation::Pointer { auto_deref: true, inner_type_name, .. } = target_type {
                // Deref auto-deref pointers before casting
                index.get_intrinsic_type_by_name(inner_type_name.as_str()).get_type_information()
            } else {
                target_type
            };

        CastInstructionData { llvm, index, llvm_type_index, value_type, target_type }
    }
}

trait Castable<'ctx, 'cast> {
    fn cast(self, cast_data: CastInstructionData<'ctx, 'cast>) -> BasicValueEnum<'ctx>;
}

trait Promotable<'ctx, 'cast> {
    fn promote(self, lsize: u32, cast_data: CastInstructionData<'ctx, 'cast>) -> BasicValueEnum<'ctx>;
}

trait Truncatable<'ctx, 'cast> {
    fn truncate(self, lsize: u32, cast_data: CastInstructionData<'ctx, 'cast>) -> BasicValueEnum<'ctx>;
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for BasicValueEnum<'ctx> {
    fn cast(self, cast_data: CastInstructionData<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        match self {
            BasicValueEnum::IntValue(val) => val.cast(cast_data),
            BasicValueEnum::FloatValue(val) => val.cast(cast_data),
            BasicValueEnum::PointerValue(val) => val.cast(cast_data),
            BasicValueEnum::ArrayValue(val) => val.cast(cast_data),
            _ => self,
        }
    }
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for IntValue<'ctx> {
    fn cast(self, cast_data: CastInstructionData<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        let lsize = cast_data.target_type.get_size_in_bits(cast_data.index);
        match cast_data.target_type {
            DataTypeInformation::Integer { .. } => {
                //its important to use the real type's size here, because we may have an i1 which is annotated as BOOL (8 bit)
                let rsize = self.get_type().get_bit_width();
                if lsize < rsize {
                    //Truncate
                    self.truncate(lsize, cast_data)
                } else {
                    //Expand
                    self.promote(lsize, cast_data)
                }
            }
            DataTypeInformation::Float { .. } => {
                let float_type = get_llvm_float_type(cast_data.llvm.context, lsize, "Float");
                if cast_data.value_type.is_signed_int() {
                    cast_data.llvm.builder.build_signed_int_to_float(self, float_type, "").into()
                } else {
                    cast_data.llvm.builder.build_unsigned_int_to_float(self, float_type, "").into()
                }
            }
            DataTypeInformation::Pointer { .. } => {
                let Ok(associated_type) = cast_data
                    .llvm_type_index
                    .get_associated_type(cast_data.target_type.get_name()) else {
                        unreachable!("Target type of cast instruction does not exist: {}", cast_data.target_type.get_name())
                    };

                cast_data.llvm.builder.build_int_to_ptr(self, associated_type.into_pointer_type(), "").into()
            }
            _ => unreachable!("Cannot cast integer value to {}", cast_data.target_type.get_name()),
        }
    }
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for FloatValue<'ctx> {
    fn cast(self, cast_data: CastInstructionData<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        let rsize = &cast_data.value_type.get_size_in_bits(cast_data.index);
        match cast_data.target_type {
            DataTypeInformation::Float { size: lsize, .. } => {
                if lsize < rsize {
                    self.truncate(*lsize, cast_data)
                } else {
                    self.promote(*lsize, cast_data)
                }
            }
            DataTypeInformation::Integer { signed, size: lsize, .. } => {
                let int_type = get_llvm_int_type(cast_data.llvm.context, *lsize, "Integer");
                if *signed {
                    cast_data.llvm.builder.build_float_to_signed_int(self, int_type, "").into()
                } else {
                    cast_data.llvm.builder.build_float_to_unsigned_int(self, int_type, "").into()
                }
            }
            _ => unreachable!("Cannot cast floating-point value to {}", cast_data.target_type.get_name()),
        }
    }
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for PointerValue<'ctx> {
    fn cast(self, cast_data: CastInstructionData<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        match &cast_data.target_type {
            DataTypeInformation::Integer { size: lsize, .. } => cast_data
                .llvm
                .builder
                .build_ptr_to_int(self, get_llvm_int_type(cast_data.llvm.context, *lsize, ""), "")
                .into(),
            DataTypeInformation::Pointer { .. } | DataTypeInformation::Void { .. } => {
                // TODO: is void really needed here? no failing tests if omitted/do we ever cast to void?
                let Ok(target_ptr_type) = cast_data.llvm_type_index.get_associated_type(cast_data.target_type.get_name()) else {
                        unreachable!("Target type of cast instruction does not exist: {}", cast_data.target_type.get_name())
                    };
                if BasicValueEnum::from(self).get_type() != target_ptr_type {
                    // bit-cast necessary
                    cast_data.llvm.builder.build_bitcast(self, target_ptr_type, "")
                } else {
                    //this is ok, no cast required
                    self.into()
                }
            }
            _ => unreachable!("Cannot cast pointer value to {}", cast_data.target_type.get_name()),
        }
    }
}

impl<'ctx, 'cast> Castable<'ctx, 'cast> for ArrayValue<'ctx> {
    fn cast(self, cast_data: CastInstructionData<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        // TODO: High-Level documentation / explanation of the following code
        if cast_data.target_type.is_vla() {
            let builder = &cast_data.llvm.builder;

            let Ok(associated_type) = cast_data
                .llvm_type_index
                .get_associated_type(cast_data.target_type.get_name()) else {
                    unreachable!("Target type of cast instruction does not exist: {}", cast_data.target_type.get_name())
                };

            // -- Generate struct & arr_ptr --
            let ty = associated_type.into_struct_type();
            let struct_vla = builder.build_alloca(ty, "local_vla");
            let struct_vla_ptr_to_arr_field = builder.build_struct_gep(struct_vla, 0, "vla_arr_ptr").unwrap();

            // Translates to
            // %1 = bitcast [6 x i32] %load_arr to i32*
            // %2 = getelementptr inbounds i32, i32* %1, i32 0
            // store i32* %2, [1 x i32]** %vla_arr_ptr, align 8
            builder.build_store(struct_vla_ptr_to_arr_field, unsafe {
                builder.build_in_bounds_gep(
                    builder
                        .build_bitcast(
                            self,
                            self.get_type()
                                .get_element_type()
                                .ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC)),
                            "",
                        )
                        .into_pointer_value(),
                    &[cast_data.llvm.i32_type().const_zero()],
                    "",
                )
            });

            // -- Generate dimensions --
            let ty = cast_data.llvm.i32_type().array_type(2);
            let dimensions_arr = builder.build_alloca(ty, "dimensions");
            let DataTypeInformation::Array { dimensions, .. } = cast_data.value_type else { unreachable!() };
            let mut dims = Vec::new();
            for dim in dimensions {
                dims.push(dim.start_offset.as_int_value(cast_data.index).unwrap());
                dims.push(dim.end_offset.as_int_value(cast_data.index).unwrap());
            }

            // Populate each array element
            for (i, val) in dims.iter().enumerate() {
                let value = cast_data.llvm.i32_type().const_int(*val as u64, true);
                let idx = cast_data.llvm.i32_type().const_int(i as u64, true);
                let adr = unsafe { builder.build_in_bounds_gep(dimensions_arr, &[idx], "") };
                builder.build_store(adr, value);
            }

            // -- Generate VLA dim ptr --
            let struct_vla_ptr_to_dim_field = builder.build_struct_gep(struct_vla, 1, "dim_arr_ptr").unwrap();
            builder.build_store(struct_vla_ptr_to_dim_field, dimensions_arr);
            return builder.build_load(struct_vla, "");
        }

        // Not a VLA, return as is
        self.into()
    }
}

impl<'ctx, 'cast> Promotable<'ctx, 'cast> for IntValue<'ctx> {
    fn promote(self, lsize: u32, cast_data: CastInstructionData<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        let llvm_int_type = get_llvm_int_type(cast_data.llvm.context, lsize, "Integer");
        if cast_data.value_type.is_signed_int() {
            cast_data.llvm.builder.build_int_s_extend_or_bit_cast(self, llvm_int_type, "")
        } else {
            cast_data.llvm.builder.build_int_z_extend_or_bit_cast(self, llvm_int_type, "")
        }
        .into()
    }
}

impl<'ctx, 'cast> Promotable<'ctx, 'cast> for FloatValue<'ctx> {
    fn promote(self, lsize: u32, cast_data: CastInstructionData<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        cast_data
            .llvm
            .builder
            .build_float_ext(self, get_llvm_float_type(cast_data.llvm.context, lsize, "Float"), "")
            .into()
    }
}

impl<'ctx, 'cast> Truncatable<'ctx, 'cast> for IntValue<'ctx> {
    fn truncate(self, lsize: u32, cast_data: CastInstructionData<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        cast_data
            .llvm
            .builder
            .build_int_truncate_or_bit_cast(
                self,
                get_llvm_int_type(cast_data.llvm.context, lsize, "Integer"),
                "",
            )
            .into()
    }
}

impl<'ctx, 'cast> Truncatable<'ctx, 'cast> for FloatValue<'ctx> {
    fn truncate(self, lsize: u32, cast_data: CastInstructionData<'ctx, 'cast>) -> BasicValueEnum<'ctx> {
        cast_data
            .llvm
            .builder
            .build_float_trunc(self, get_llvm_float_type(cast_data.llvm.context, lsize, "Float"), "")
            .into()
    }
}
