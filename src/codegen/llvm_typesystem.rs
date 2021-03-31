/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use inkwell::{builder::Builder, context::Context, types::{FloatType, IntType}, values::{BasicValueEnum, IntValue}};

use crate::{index::Index, ast::Statement, compile_error::CompileError, typesystem::{get_bigger_type,DataTypeInformation}};

use super::{TypeAndValue, generators::llvm::LLVM, llvm_index::LLVMTypedIndex};

pub fn promote_if_needed<'a>(
    context : &'a Context,
    builder: &Builder<'a>,
    lvalue: &TypeAndValue<'a>,
    rvalue: &TypeAndValue<'a>,
    index : &Index,
    llvm_index: &LLVMTypedIndex<'a>,
) -> (
    DataTypeInformation,
    BasicValueEnum<'a>,
    BasicValueEnum<'a>,
) {
    let (ltype, lvalue) = lvalue;
    let (rtype, rvalue) = rvalue;

    let ltype_llvm = llvm_index.find_associated_type(ltype.get_name()).unwrap();
    let rtype_llvm = llvm_index.find_associated_type(rtype.get_name()).unwrap();

    if ltype.is_numerical() && rtype.is_numerical() {
        if ltype_llvm == rtype_llvm {
            (ltype.clone(), *lvalue, *rvalue)
        } else {
            let target_type = get_bigger_type(
                &get_bigger_type(ltype, rtype),
                &index.find_type_information("DINT").unwrap()
            );

            let promoted_lvalue = promote_value_if_needed(context, builder, *lvalue, ltype, &target_type);
            let promoted_rvalue = promote_value_if_needed(context, builder, *rvalue, rtype, &target_type);

            return (target_type, promoted_lvalue, promoted_rvalue);
        }
    } else {
        panic!("Binary operations need numerical types")
    }
}



fn promote_value_if_needed<'ctx>(
   context : &'ctx Context,
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
           size: target_size,
           ..
       } => {
           // INT --> INT
           let int_value = lvalue.into_int_value();
           if int_value.get_type().get_bit_width() < *target_size {
               create_llvm_extend_int_value(
                   builder,
                   int_value,
                   ltype,
                   get_llvm_int_type(context, *target_size, "Integer").unwrap(),
               ).into()
           } else {
               lvalue
           }
       }
       DataTypeInformation::Float {
           size: target_size,
           ..
       } => {
           if let DataTypeInformation::Integer { signed, .. } = ltype {
               // INT --> FLOAT
               let int_value = lvalue.into_int_value();
               if *signed {
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
       DataTypeInformation::Integer {
           signed: true,
           ..
       } => builder.build_int_s_extend_or_bit_cast(lvalue, target_type, ""),
       DataTypeInformation::Integer {
           signed: false,
           ..
       } => builder.build_int_z_extend_or_bit_cast(lvalue, target_type, ""),
       _ => unreachable!(),
   }
}

pub fn cast_if_needed<'ctx>(
   llvm: &LLVM<'ctx>,
   target_type: &DataTypeInformation,
   value: BasicValueEnum<'ctx>,
   value_type: &DataTypeInformation,
   location_context: &Statement,
) -> Result<BasicValueEnum<'ctx>, CompileError> {
   let builder = &llvm.builder;
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
                       Ok(llvm.builder
                           .build_int_truncate_or_bit_cast(
                               value.into_int_value(),
                               get_llvm_int_type(llvm.context, *lsize, "Integer").unwrap(),
                               "",
                           )
                           .into())
                   } else {
                       //Expand
                       Ok(
                           promote_value_if_needed(llvm.context, &llvm.builder, value, value_type, &target_type)
                               .into(),
                       )
                   }
               }
               DataTypeInformation::Float {
                   size: _rsize,
                   ..
               } => {
                   if *signed {
                       Ok(llvm.builder
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
                       &value_type.get_name(),
                       &target_type.get_name(),
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
                       &llvm.context,
                       &llvm.builder,
                       value,
                       value_type,
                       &target_type,
                   ))
               }
           }
           _ => Err(CompileError::casting_error(
               &value_type.get_name(),
               &target_type.get_name(),
               location_context.get_location(),
           )),
       },
       DataTypeInformation::String { size, .. } => match value_type {
           DataTypeInformation::String {
               size: value_size, ..
           } => {
               if size < value_size {
                   //if we are on a vector replace it
                   if value.is_vector_value() {
                       let vec_value = value.into_vector_value();
                       let string_value = vec_value.get_string_constant().to_bytes();
                       let new_value= &string_value[0..(*size -1) as usize];
                       let (_,value) = llvm.create_llvm_const_vec_string(new_value)?;
                       Ok(value)
                   }
                   else {
                       Err(CompileError::casting_error(
                                       &value_type.get_name(),
                                       &target_type.get_name(),
                                       location_context.get_location()))
                   }
               } else {
                   Ok(value)
               }
           }
           _ => Err(CompileError::casting_error(
               &value_type.get_name(),
               &target_type.get_name(),
               location_context.get_location(),
           )),
       },
       _ => Ok(value),
   }
}

pub fn get_llvm_int_type<'a>(context: &'a Context, size : u32, name : &str) -> Result<IntType<'a>,CompileError>{
    match size {
        1 => Ok(context.bool_type()),
        8 => Ok(context.i8_type()),
        16 => Ok(context.i16_type()),
        32 => Ok(context.i32_type()),
        64 => Ok(context.i64_type()),
        128 => Ok(context.i128_type()),
        _ => Err(CompileError::codegen_error(format!("Invalid size for type : '{}' at {}",name, size), 0..0)),
    }

}

    pub fn get_llvm_float_type<'a>(context: &'a Context, size : u32, name : &str) -> Result<FloatType<'a>,CompileError>{
    match size {
        32 => Ok(context.f32_type()),
        64 => Ok(context.f64_type()),
        _ => Err(CompileError::codegen_error(format!("Invalid size for type : '{}' at {}",name, size), 0..0)),
    }

}
