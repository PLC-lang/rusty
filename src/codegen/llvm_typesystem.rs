// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use inkwell::{
    builder::Builder,
    context::Context,
    types::{FloatType, IntType},
    values::{BasicValueEnum, IntValue},
};

use crate::{
    ast::AstStatement,
    ast::SourceRange,
    compile_error::CompileError,
    index::Index,
    typesystem::{DataType, DataTypeInformation, StringEncoding},
};

use super::generators::llvm::Llvm;

pub fn promote_value_if_needed<'ctx>(
    context: &'ctx Context,
    builder: &Builder<'ctx>,
    lvalue: BasicValueEnum<'ctx>,
    ltype: &DataTypeInformation,
    target_type: &DataTypeInformation,
) -> Result<BasicValueEnum<'ctx>, CompileError> {
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
                Ok(create_llvm_extend_int_value(
                    builder,
                    int_value,
                    ltype,
                    get_llvm_int_type(context, *target_size, "Integer")?,
                )
                .into())
            } else {
                Ok(lvalue)
            }
        }
        DataTypeInformation::Float {
            size: target_size, ..
        } => {
            if lvalue.is_int_value() {
                // INT --> FLOAT
                let int_value = lvalue.into_int_value();
                if ltype.is_signed_int() {
                    Ok(builder
                        .build_signed_int_to_float(
                            int_value,
                            get_llvm_float_type(context, *target_size, "Float")?,
                            "",
                        )
                        .into())
                } else {
                    Ok(builder
                        .build_unsigned_int_to_float(
                            int_value,
                            get_llvm_float_type(context, *target_size, "Float")?,
                            "",
                        )
                        .into())
                }
            } else {
                // FLOAT --> FLOAT
                if let DataTypeInformation::Float { size, .. } = ltype {
                    if target_size <= size {
                        Ok(lvalue)
                    } else {
                        Ok(builder
                            .build_float_ext(
                                lvalue.into_float_value(),
                                get_llvm_float_type(context, *target_size, "Float")?,
                                "",
                            )
                            .into())
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
    target_type: &DataType,
    value: BasicValueEnum<'ctx>,
    value_type: &DataType,
    //TODO: Could be location
    statement: &AstStatement,
) -> Result<BasicValueEnum<'ctx>, CompileError> {
    let builder = &llvm.builder;
    let target_type = index
        .find_effective_type_info(target_type.get_name())
        .unwrap_or_else(|| index.get_void_type().get_type_information());

    let value_type = index
        .find_effective_type_info(value_type.get_name())
        .unwrap_or_else(|| index.get_void_type().get_type_information());

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
                                get_llvm_int_type(llvm.context, *lsize, "Integer")?,
                                "",
                            )
                            .into())
                    } else {
                        //Expand
                        promote_value_if_needed(
                            llvm.context,
                            &llvm.builder,
                            value,
                            value_type,
                            target_type,
                        )
                        .map_err(|it| CompileError::relocate(&it, statement.get_location()))
                    }
                }
                DataTypeInformation::Float { size: _rsize, .. } => {
                    if *signed {
                        Ok(llvm
                            .builder
                            .build_float_to_signed_int(
                                value.into_float_value(),
                                get_llvm_int_type(llvm.context, *lsize, "Integer")?,
                                "",
                            )
                            .into())
                    } else {
                        Ok(builder
                            .build_float_to_unsigned_int(
                                value.into_float_value(),
                                get_llvm_int_type(llvm.context, *lsize, "Integer")?,
                                "",
                            )
                            .into())
                    }
                }
                DataTypeInformation::String { encoding, .. } => {
                    if (*lsize == 8 && matches!(encoding, StringEncoding::Utf16))
                        || (*lsize == 16 && matches!(encoding, StringEncoding::Utf8))
                    {
                        return Err(CompileError::casting_error(
                            value_type.get_name(),
                            target_type.get_name(),
                            statement.get_location(),
                        ));
                    };
                    Ok(llvm
                        .builder
                        .build_int_truncate_or_bit_cast(
                            value.into_int_value(),
                            get_llvm_int_type(llvm.context, *lsize, "Integer").unwrap(),
                            "",
                        )
                        .into())
                }
                _ => Err(CompileError::casting_error(
                    value_type.get_name(),
                    target_type.get_name(),
                    statement.get_location(),
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
                            get_llvm_float_type(llvm.context, *lsize, "Float")?,
                            "",
                        )
                        .into())
                } else {
                    Ok(builder
                        .build_unsigned_int_to_float(
                            value.into_int_value(),
                            get_llvm_float_type(llvm.context, *lsize, "Float")?,
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
                            get_llvm_float_type(llvm.context, *lsize, "Float")?,
                            "",
                        )
                        .into())
                } else {
                    promote_value_if_needed(
                        llvm.context,
                        &llvm.builder,
                        value,
                        value_type,
                        target_type,
                    )
                    .map_err(|it| CompileError::relocate(&it, statement.get_location()))
                }
            }
            _ => Err(CompileError::casting_error(
                value_type.get_name(),
                target_type.get_name(),
                statement.get_location(),
            )),
        },
        DataTypeInformation::String { size, encoding } => match value_type {
            DataTypeInformation::String {
                size: value_size,
                encoding: value_encoding,
            } => {
                if encoding != value_encoding {
                    return Err(CompileError::casting_error(
                        value_type.get_name(),
                        target_type.get_name(),
                        statement.get_location(),
                    ));
                }
                let size = size
                    .as_int_value(index)
                    .map_err(|msg| CompileError::codegen_error(msg, SourceRange::undefined()))?
                    as u32;
                let value_size = value_size
                    .as_int_value(index)
                    .map_err(|msg| CompileError::codegen_error(msg, SourceRange::undefined()))?
                    as u32;

                if size < value_size {
                    //we need to downcast the size of the string
                    //check if it's a literal, if so we can exactly know how big this is
                    if let AstStatement::LiteralString {
                        is_wide,
                        value: string_value,
                        ..
                    } = statement
                    {
                        let value = if *is_wide {
                            let mut chars = string_value.encode_utf16().collect::<Vec<u16>>();
                            //We add a null terminator since the llvm command will not account for
                            //it
                            chars.push(0);
                            let total_bytes_to_copy = std::cmp::min(size, chars.len() as u32);
                            let new_value = &chars[0..(total_bytes_to_copy) as usize];
                            llvm.create_llvm_const_utf16_vec_string(new_value)?
                        } else {
                            let bytes = string_value.bytes().collect::<Vec<u8>>();
                            let total_bytes_to_copy = std::cmp::min(size - 1, bytes.len() as u32);
                            let new_value = &bytes[0..total_bytes_to_copy as usize];
                            //This accounts for a null terminator, hence we don't add it here.
                            llvm.create_llvm_const_vec_string(new_value)?
                        };
                        Ok(value)
                    } else {
                        //if we are on a vector replace it
                        if value.is_vector_value() {
                            let vec_value = value.into_vector_value();
                            let string_value = vec_value.get_string_constant().to_bytes();
                            let real_size = std::cmp::min(size, (string_value.len() + 1) as u32);
                            if real_size < value_size {
                                let new_value = &string_value[0..(real_size - 1) as usize];
                                let value = llvm.create_llvm_const_vec_string(new_value)?;
                                Ok(value)
                            } else {
                                Ok(value)
                            }
                        } else {
                            unreachable!()
                        }
                    }
                } else {
                    Ok(value)
                }
            }
            _ => Err(CompileError::casting_error(
                value_type.get_name(),
                target_type.get_name(),
                statement.get_location(),
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
