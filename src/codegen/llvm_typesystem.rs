// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use inkwell::{builder::Builder, context::Context, types::{FloatType, IntType}, values::{BasicValue, BasicValueEnum, IntValue}};

use crate::{ast::AstStatement, ast::SourceRange, compile_error::CompileError, index::Index, typesystem::{DataType, DataTypeInformation, get_bigger_type}};

use super::{generators::llvm::Llvm, llvm_index::LlvmTypedIndex, TypeAndValue};

pub fn promote_if_needed<'a>(
    context: &'a Context,
    builder: &Builder<'a>,
    lvalue: &TypeAndValue<'a>,
    rvalue: &TypeAndValue<'a>,
    index: &Index,
    llvm_index: &LlvmTypedIndex<'a>,
) -> (DataTypeInformation, BasicValueEnum<'a>, BasicValueEnum<'a>) {
    let (ltype, lvalue) = lvalue;
    let (rtype, rvalue) = rvalue;

    //TODO : We need better error handling here
    let ltype_llvm = llvm_index.find_associated_type(ltype.get_name()).unwrap();
    let rtype_llvm = llvm_index.find_associated_type(rtype.get_name()).unwrap();

    if ltype.is_numerical() && rtype.is_numerical() {
        if ltype_llvm == rtype_llvm {
            (ltype.clone(), *lvalue, *rvalue)
        } else {
            let target_type = get_bigger_type(
                &get_bigger_type(ltype, rtype),
                &index.find_type_information("DINT").unwrap(),
            );

            let promoted_lvalue =
                promote_value_if_needed(context, builder, *lvalue, ltype, &target_type);
            let promoted_rvalue =
                promote_value_if_needed(context, builder, *rvalue, rtype, &target_type);

            (target_type, promoted_lvalue, promoted_rvalue)
        }
    } else {
        dbg!(ltype);
        dbg!(rtype);
        panic!("Binary operations need numerical types")
    }
}

pub fn promote_value_if_needed<'ctx>(
    context: &'ctx Context,
    builder: &Builder<'ctx>,
    lvalue: BasicValueEnum<'ctx>,
    ltype: &DataTypeInformation,
    target_type: &DataTypeInformation,
) -> BasicValueEnum<'ctx> {
    //Is the target type int
    //Expand the current type to the target size
    //Is the target type float
    //Is the current type int
    //Cast to float
    //Expand current type to target type

    match target_type {
        DataTypeInformation::Integer {
            size: target_size, ..
        } => {
            // INT --> INT
            let int_value = lvalue.into_int_value();
            if int_value.get_type().get_bit_width() < *target_size {
                create_llvm_extend_int_value(
                    builder,
                    int_value,
                    ltype,
                    get_llvm_int_type(context, *target_size, "Integer").unwrap(),
                )
                .into()
            } else {
                lvalue
            }
        }
        DataTypeInformation::Float {
            size: target_size, ..
        } => {
            if lvalue.is_int_value() {
                // INT --> FLOAT
                let int_value = lvalue.into_int_value();
                if ltype.is_signed_int() {
                    builder
                        .build_signed_int_to_float(
                            int_value,
                            get_llvm_float_type(context, *target_size, "Float").unwrap(),
                            "",
                        )
                        .into()
                } else {
                    builder
                        .build_unsigned_int_to_float(
                            int_value,
                            get_llvm_float_type(context, *target_size, "Float").unwrap(),
                            "",
                        )
                        .into()
                }
            } else {
                // FLOAT --> FLOAT
                if let DataTypeInformation::Float { size, .. } = ltype {
                    if target_size <= size {
                        lvalue
                    } else {
                        builder
                            .build_float_ext(
                                lvalue.into_float_value(),
                                get_llvm_float_type(context, *target_size, "Float").unwrap(),
                                "",
                            )
                            .into()
                    }
                } else {
                    unreachable!()
                }
            }
        }
        _ => unreachable!(),
    }
}

fn create_llvm_extend_int_value<'a>(
    builder: &Builder<'a>,
    lvalue: IntValue<'a>,
    ltype: &DataTypeInformation,
    target_type: IntType<'a>,
) -> IntValue<'a> {
    match ltype {
        DataTypeInformation::Integer { signed: true, .. } => {
            builder.build_int_s_extend_or_bit_cast(lvalue, target_type, "")
        }
        DataTypeInformation::Integer { signed: false, .. } => {
            builder.build_int_z_extend_or_bit_cast(lvalue, target_type, "")
        }
        _ => unreachable!(),
    }
}

pub fn cast_if_needed<'ctx>(
    llvm: &Llvm<'ctx>,
    index: &Index,
    target_type: &DataTypeInformation,
    value: BasicValueEnum<'ctx>,
    value_type: &DataTypeInformation,
    location_context: &AstStatement,
) -> Result<BasicValueEnum<'ctx>, CompileError> {
    let builder = &llvm.builder;
    let target_type = index
        .find_effective_type_information(target_type)
        .ok_or_else(|| {
            CompileError::codegen_error(
                format!("Could not find primitive type for {:?}", target_type),
                SourceRange::undefined(),
            )
        })?;
    let value_type = index
        .find_effective_type_information(value_type)
        .ok_or_else(|| {
            CompileError::codegen_error(
                format!("Could not find primitive type for {:?}", value_type),
                SourceRange::undefined(),
            )
        })?;
    match target_type {
        DataTypeInformation::Integer {
            signed,
            size: lsize,
            ..
        } => {
            match value_type {
                DataTypeInformation::Integer { size: rsize, .. } => {
                    if lsize < rsize {
                        //Truncate
                        Ok(llvm
                            .builder
                            .build_int_truncate_or_bit_cast(
                                value.into_int_value(),
                                get_llvm_int_type(llvm.context, *lsize, "Integer").unwrap(),
                                "",
                            )
                            .into())
                    } else {
                        //Expand
                        Ok(promote_value_if_needed(
                            llvm.context,
                            &llvm.builder,
                            value,
                            value_type,
                            target_type,
                        ))
                    }
                }
                DataTypeInformation::Float { size: _rsize, .. } => {
                    if *signed {
                        Ok(llvm
                            .builder
                            .build_float_to_signed_int(
                                value.into_float_value(),
                                get_llvm_int_type(llvm.context, *lsize, "Integer").unwrap(),
                                "",
                            )
                            .into())
                    } else {
                        Ok(builder
                            .build_float_to_unsigned_int(
                                value.into_float_value(),
                                get_llvm_int_type(llvm.context, *lsize, "Integer").unwrap(),
                                "",
                            )
                            .into())
                    }
                }
                _ => Err(CompileError::casting_error(
                    value_type.get_name(),
                    target_type.get_name(),
                    location_context.get_location(),
                )),
            }
        }
        
        DataTypeInformation::Float {
            // generated_type,
            size: lsize,
            ..
        } => match value_type {
            DataTypeInformation::Integer { signed, .. } => {
                if *signed {
                    Ok(builder
                        .build_signed_int_to_float(
                            value.into_int_value(),
                            get_llvm_float_type(llvm.context, *lsize, "Float").unwrap(),
                            "",
                        )
                        .into())
                } else {
                    Ok(builder
                        .build_unsigned_int_to_float(
                            value.into_int_value(),
                            get_llvm_float_type(llvm.context, *lsize, "Float").unwrap(),
                            "",
                        )
                        .into())
                }
            }
            DataTypeInformation::Float { size: rsize, .. } => {
                if lsize < rsize {
                    Ok(builder
                        .build_float_trunc(
                            value.into_float_value(),
                            get_llvm_float_type(llvm.context, *lsize, "Float").unwrap(),
                            "",
                        )
                        .into())
                } else {
                    Ok(promote_value_if_needed(
                        llvm.context,
                        &llvm.builder,
                        value,
                        value_type,
                        target_type,
                    ))
                }
            }
            _ => Err(CompileError::casting_error(
                value_type.get_name(),
                target_type.get_name(),
                location_context.get_location(),
            )),
        },
        DataTypeInformation::String { size, .. } => match value_type {
            DataTypeInformation::String {
                size: value_size, ..
            } => {
                let size = size
                    .as_int_value(index)
                    .map_err(|msg| CompileError::codegen_error(msg, SourceRange::undefined()))?
                    as u32;
                let value_size = value_size
                    .as_int_value(index)
                    .map_err(|msg| CompileError::codegen_error(msg, SourceRange::undefined()))?
                    as u32;
   
                if size < value_size {
                    //if we are on a vector replace it
                    if value.is_vector_value() {
                        let vec_value = value.into_vector_value();
                        let string_value = vec_value.get_string_constant().to_bytes();
                        let real_size = std::cmp::min(size, (string_value.len() +1) as u32);
                        if real_size < value_size {

                            let new_value = &string_value[0..(real_size - 1) as usize];
                            let (_, value) = llvm.create_llvm_const_vec_string(new_value)?;
                            Ok(value)
                        }else{
                            Ok(value)
                        }
                    } else {
                        Err(CompileError::casting_error(
                            value_type.get_name(),
                            target_type.get_name(),
                            location_context.get_location(),
                        ))
                    }
                } else {
                    Ok(value)
                }
            }
            _ => Err(CompileError::casting_error(
                value_type.get_name(),
                target_type.get_name(),
                location_context.get_location(),
            )),
        },
        _ => Ok(value),
    }
}

pub fn get_llvm_int_type<'a>(
    context: &'a Context,
    size: u32,
    name: &str,
) -> Result<IntType<'a>, CompileError> {
    match size {
        1 => Ok(context.bool_type()),
        8 => Ok(context.i8_type()),
        16 => Ok(context.i16_type()),
        32 => Ok(context.i32_type()),
        64 => Ok(context.i64_type()),
        128 => Ok(context.i128_type()),
        _ => Err(CompileError::codegen_error(
            format!("Invalid size for type : '{}' at {}", name, size),
            SourceRange::undefined(),
        )),
    }
}

pub fn get_llvm_float_type<'a>(
    context: &'a Context,
    size: u32,
    name: &str,
) -> Result<FloatType<'a>, CompileError> {
    match size {
        32 => Ok(context.f32_type()),
        64 => Ok(context.f64_type()),
        _ => Err(CompileError::codegen_error(
            format!("Invalid size for type : '{}' at {}", name, size),
            SourceRange::undefined(),
        )),
    }
}
