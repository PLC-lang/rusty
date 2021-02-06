/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use inkwell::{
    context::Context,
    types::{BasicType, BasicTypeEnum, StringRadix},
    values::{BasicValueEnum},
};

use crate::{compile_error::CompileError, index::Index};

use super::typesystem;
use super::{CodeGen, ExpressionValue, TypeAndValue};

pub fn create_llvm_const_bool<'a>(
    context: &'a Context,
    index: &Index<'a>,
    value: bool,
) -> Result<TypeAndValue<'a>, CompileError> {
    let itype = context.bool_type();
    let value = itype.const_int(value as u64, false);

    let data_type = index.get_type_information("BOOL")?;
    Ok((data_type, BasicValueEnum::IntValue(value)))
}
pub fn create_llvm_const_int<'a>(
    context: &'a Context,
    index: &Index<'a>,
    expected_type: &Option<BasicTypeEnum<'a>>,
    value: &str,
) -> Result<TypeAndValue<'a>, CompileError> {
    let i32_type = context.i32_type().as_basic_type_enum();
    let data_type = expected_type.unwrap_or(i32_type);

    if let BasicTypeEnum::IntType { 0: int_type } = data_type {
        let value = int_type.const_int_from_string(value, StringRadix::Decimal);
        let data_type = index.get_type_information("DINT")?;

        Ok((data_type, BasicValueEnum::IntValue(value.unwrap())))
    } else {
        panic!("error expected inttype");
    }
}

pub fn create_llvm_const_real<'a>(
    context: &'a Context,
    index: &Index<'a>,
    expected_type: &Option<BasicTypeEnum<'a>>,
    value: &str,
) -> Result<TypeAndValue<'a>, CompileError> {
    let double_type = context.f32_type().as_basic_type_enum();
    let data_type = expected_type.unwrap_or(double_type);

    if let BasicTypeEnum::FloatType { 0: float_type } = data_type {
        let value = float_type.const_float_from_string(value);
        let data_type = index.get_type_information("REAL")?;
        Ok((data_type, BasicValueEnum::FloatValue(value)))
    } else {
        panic!("error expected floattype")
    }
}

pub fn create_llvm_const_string<'a>(
    context: &'a Context,
    value: &str,
) -> Result<TypeAndValue<'a>, CompileError> {
    create_llvm_const_vec_string(context, value.as_bytes())
}

pub fn create_llvm_const_vec_string<'a>(
    context: &'a Context,
    value: &[u8],
) -> Result<TypeAndValue<'a>, CompileError> {
    let exp_value = context.const_string(value, true);
    Ok((
        typesystem::new_string_information(context, value.len() as u32),
        BasicValueEnum::VectorValue(exp_value),
    ))
}

impl<'ctx> CodeGen<'ctx> {
    pub fn generate_literal_integer<'a>(
        llvm: &'a Context,
        index: &'a Index,
        current_type: &'a Option<BasicTypeEnum>,
        value: &str,
    ) -> ExpressionValue<'a> {
        let i32_type = llvm.i32_type().as_basic_type_enum();
        let data_type = current_type.unwrap_or(i32_type);

        if let BasicTypeEnum::IntType { 0: int_type } = data_type {
            let value = int_type.const_int_from_string(value, StringRadix::Decimal);
            let data_type = index.find_type_information("DINT");
            (data_type, Some(BasicValueEnum::IntValue(value.unwrap())))
        } else {
            panic!("error expected inttype");
        }
    }

    pub fn generate_literal_real<'a>(
        llvm: &'a Context,
        index: &'a Index,
        current_type: &'a Option<BasicTypeEnum>,
        value: &str,
    ) -> ExpressionValue<'a> {
        let double_type = llvm.f32_type().as_basic_type_enum();
        let data_type = current_type.unwrap_or(double_type);

        if let BasicTypeEnum::FloatType { 0: float_type } = data_type {
            let value = float_type.const_float_from_string(value);
            let data_type = index.find_type_information("REAL");
            (data_type, Some(BasicValueEnum::FloatValue(value)))
        } else {
            panic!("error expected floattype")
        }
    }

    pub fn generate_literal_boolean<'a>(
        llvm: &'a Context,
        index: &'a Index,
        value: bool,
    ) -> ExpressionValue<'a> {
        let itype = llvm.bool_type();
        let value = itype.const_int(value as u64, false);
        let data_type = index.find_type_information("BOOL");
        (data_type, Some(BasicValueEnum::IntValue(value)))
    }

    pub fn generate_literal_string<'a>(llvm: &'a Context, value: &[u8]) -> ExpressionValue<'a> {
        let exp_value = llvm.const_string(value, true);
        (
            Some(typesystem::new_string_information(llvm, value.len() as u32)),
            Some(exp_value.into()),
        )
    }

    /*
    pub fn generate_literal_array(
        &mut self,
        value: &Statement,
    ) -> Result<BasicValueEnum, String> {
        let inner_type = self.context.get_current_type()
               .ok_or("cannot derive type when generating array literal.")?;

        if let BasicTypeEnum::ArrayType(at) = inner_type {
            let element_type = at.get_element_type();

            let values = if let Statement::ExpressionList { expressions } = value {
                expressions.iter().collect()
            } else {
                vec!(value)
            };

            self.context.push_type_hint(element_type);
            let mut array_values = Vec::new();
            for s in values {
                let (_, value) = self.generate_statement(&s)?;
                array_values.push(value.ok_or("No value")?);
            }
            todo!("continue!");
            self.context.pop_type_hint();

            generate_array_value(element_type, at.len(), &array_values)
        } else {
            Err(format!(
                "Expected array type context, but found {:?}",
                inner_type
            ))
        }
    }

    fn generate_array_value<'ctx>(
        element_type: BasicTypeEnum<'ctx>,
        len: u32,
        values: &Vec<BasicValueEnum<'ctx>>,
        offset: Range<usize>
    ) -> Result<BasicValueEnum<'ctx>, CompileError> {
        match element_type {
            /*BasicTypeEnum::ArrayType(t) => {
                let array = t.get_undef();
                for (i, v) in values.iter().enumerate() {
                    if let BasicValueEnum::ArrayValue(av) = v {
                        //TODO what should I do with the returned value?
                        array.const_insert_value(*av, &mut [i as u32]);
                    } else {
                        return Err(format!(
                            "expected {:?} but found {:?} when generating values.",
                            element_type, v
                        ));
                    }
                }
                Ok(array.into())
            }*/
            BasicTypeEnum::IntType(t) => {
                let array = t.array_type(len).get_undef();
                for (i, v) in values.iter().enumerate() {
                    if let BasicValueEnum::IntValue(iv) = v {
                        //TODO what should I do with the returned value?
                        array.const_insert_value(*iv, &mut [i as u32]);
                    } else {
                        return Err(CompileError::CodeGenError{ message: format!(
                            "expected {:?} but found {:?} when generating values.",
                            element_type, v), location: offset});
                        }
                    }
                    Ok(array.clone().into())
                }
                _ => panic!("aaaaah"),
            }
        }*/
}
