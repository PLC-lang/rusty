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
    diagnostics::Diagnostic,
    index::Index,
    typesystem::{DataType, DataTypeInformation, StringEncoding},
};

use super::{generators::llvm::Llvm, llvm_index::LlvmTypedIndex};

pub fn promote_value_if_needed<'ctx>(
    context: &'ctx Context,
    builder: &Builder<'ctx>,
    lvalue: BasicValueEnum<'ctx>,
    ltype: &DataTypeInformation,
    target_type: &DataTypeInformation,
) -> Result<BasicValueEnum<'ctx>, Diagnostic> {
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

pub fn create_llvm_extend_int_value<'a>(
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

///
/// generates a cast from the given `value` to the given `target_type` if necessary and returns the casted value. It returns
/// the original `value` if no cast is necessary
///
/// - `llvm` the llvm utilities to use for code-generation
/// - `index` the current Index used for type-lookups
/// - `llvm_type_index` the type index to lookup llvm generated types
/// - `target_type` the expected target type of the value
/// - `value` the value to (maybe) cast
/// - `value_type` the current type of the given value
/// - `statement` the original statement as a context (e.g. for error reporting)
///
pub fn cast_if_needed<'ctx>(
    llvm: &Llvm<'ctx>,
    index: &Index,
    llvm_type_index: &LlvmTypedIndex<'ctx>,
    target_type: &DataType,
    value: BasicValueEnum<'ctx>,
    value_type: &DataType,
    //TODO: Could be location
    statement: &AstStatement,
) -> Result<BasicValueEnum<'ctx>, Diagnostic> {
    let builder = &llvm.builder;
    let target_type = index
        .get_intrinsic_type_by_name(target_type.get_name())
        .get_type_information();

    let value_type = index
        .get_intrinsic_type_by_name(value_type.get_name())
        .get_type_information();

    // if the current or target type are generic (unresolved or builtin)
    // return the value without modification
    if target_type.is_generic(index) || value_type.is_generic(index) {
        return Ok(value);
    }

    match target_type {
        DataTypeInformation::Integer {
            signed,
            size: lsize,
            ..
        } => {
            match value_type {
                DataTypeInformation::Integer { .. } => {
                    //its important to use the real type's size here, because we may bot an i1 which is annotated as BOOL (8 bit)
                    let rsize = &value.get_type().into_int_type().get_bit_width();
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
                        .map_err(|it| Diagnostic::relocate(it, statement.get_location()))
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
                        return Err(Diagnostic::casting_error(
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
                DataTypeInformation::Pointer {
                    auto_deref: false, ..
                } => Ok(llvm
                    .builder
                    .build_ptr_to_int(
                        value.into_pointer_value(),
                        get_llvm_int_type(llvm.context, *lsize, "")?,
                        "",
                    )
                    .into()),
                _ => Err(Diagnostic::casting_error(
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
                    .map_err(|it| Diagnostic::relocate(it, statement.get_location()))
                }
            }
            _ => Err(Diagnostic::casting_error(
                value_type.get_name(),
                target_type.get_name(),
                statement.get_location(),
            )),
        },
        DataTypeInformation::String { encoding, .. } => match value_type {
            DataTypeInformation::String {
                encoding: value_encoding,
                ..
            } => {
                if encoding != value_encoding {
                    return Err(Diagnostic::casting_error(
                        value_type.get_name(),
                        target_type.get_name(),
                        statement.get_location(),
                    ));
                }
                Ok(value)
            }
            _ => Err(Diagnostic::casting_error(
                value_type.get_name(),
                target_type.get_name(),
                statement.get_location(),
            )),
        },
        DataTypeInformation::Pointer {
            auto_deref: false, ..
        } => match value_type {
            DataTypeInformation::Integer { .. } => Ok(llvm
                .builder
                .build_int_to_ptr(
                    value.into_int_value(),
                    llvm_type_index
                        .get_associated_type(target_type.get_name())?
                        .into_pointer_type(),
                    "",
                )
                .into()),
            DataTypeInformation::Pointer { .. } | DataTypeInformation::Void { .. } => {
                let target_ptr_type =
                    llvm_type_index.get_associated_type(target_type.get_name())?;
                if value.get_type() != target_ptr_type {
                    // bit-cast necessary
                    Ok(builder.build_bitcast(value, target_ptr_type, ""))
                } else {
                    //this is ok, no cast required
                    Ok(value)
                }
            }
            _ => Err(Diagnostic::casting_error(
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
) -> Result<IntType<'a>, Diagnostic> {
    match size {
        1 => Ok(context.bool_type()),
        8 => Ok(context.i8_type()),
        16 => Ok(context.i16_type()),
        32 => Ok(context.i32_type()),
        64 => Ok(context.i64_type()),
        128 => Ok(context.i128_type()),
        _ => Err(Diagnostic::codegen_error(
            &format!("Invalid size for type : '{}' at {}", name, size),
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
            &format!("Invalid size for type : '{}' at {}", name, size),
            SourceRange::undefined(),
        )),
    }
}
