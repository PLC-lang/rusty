/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use inkwell::{FloatPredicate, IntPredicate, builder::Builder, values::{BasicValueEnum}};

use crate::{ast::Operator, index::{DataTypeInformation, Index}};

use super::TypeAndValue;


pub fn create_llvm_int_binary_expression<'a>(
        builder: &Builder<'a>,
        index: &Index<'a>,
        operator: &Operator,
        left_value: BasicValueEnum<'a>,
        right_value: BasicValueEnum<'a>,
        target_type: &DataTypeInformation<'a>,
    ) -> TypeAndValue<'a> {
        let int_lvalue = left_value.into_int_value();
        let int_rvalue = right_value.into_int_value();

        let (value, data_type) = match operator {
            Operator::Plus => (
                builder
                    .build_int_add(int_lvalue, int_rvalue, "tmpVar"),
                target_type.clone(),
            ),
            Operator::Minus => (
                builder
                    .build_int_sub(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Multiplication => (
                builder
                    .build_int_mul(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Division => (
                builder
                    .build_int_signed_div(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Modulo => (
                builder
                    .build_int_signed_rem(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Equal => (
                builder
                    .build_int_compare(IntPredicate::EQ, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                index.find_type_information("BOOL").unwrap()
            ),

            Operator::NotEqual => (
                builder
                    .build_int_compare(IntPredicate::NE, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                index.find_type_information("BOOL").unwrap(),
            ),

            Operator::Less => (
                builder
                    .build_int_compare(IntPredicate::SLT, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                index.find_type_information("BOOL").unwrap(),
            ),

            Operator::Greater => (
                builder
                    .build_int_compare(IntPredicate::SGT, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                index.find_type_information("BOOL").unwrap(),
            ),

            Operator::LessOrEqual => (
                builder
                    .build_int_compare(IntPredicate::SLE, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                index.find_type_information("BOOL").unwrap(),
            ),

            Operator::GreaterOrEqual => (
                builder
                    .build_int_compare(IntPredicate::SGE, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                index.find_type_information("BOOL").unwrap(),
            ),
            Operator::Xor => (
                builder
                    .build_xor(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                index.find_type_information("BOOL").unwrap(),
            ),
            _ => unimplemented!(),
        };
        (data_type, value.into())
    }

pub fn create_llvm_float_binary_expression<'a>(
        builder: &Builder<'a>,
        index: &Index<'a>,
        operator: &Operator,
        lvalue: BasicValueEnum<'a>,
        rvalue: BasicValueEnum<'a>,
        target_type: &DataTypeInformation<'a>,
    ) -> TypeAndValue<'a> {
        let int_lvalue = lvalue.into_float_value();
        let int_rvalue = rvalue.into_float_value();

        let (value, data_type) = match operator {
            Operator::Plus => (
                builder
                    .build_float_add(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Minus => (
                builder
                    .build_float_sub(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Multiplication => (
                builder
                    .build_float_mul(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Division => (
                builder
                    .build_float_div(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Modulo => (
                builder
                    .build_float_rem(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Equal => (
                builder
                    .build_float_compare(FloatPredicate::OEQ, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                index.find_type_information("BOOL").unwrap(),
            ),

            Operator::NotEqual => (
                builder
                    .build_float_compare(FloatPredicate::ONE, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                index.find_type_information("BOOL").unwrap(),
            ),

            Operator::Less => (
                builder
                    .build_float_compare(FloatPredicate::OLT, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                index.find_type_information("BOOL").unwrap(),
            ),

            Operator::Greater => (
                builder
                    .build_float_compare(FloatPredicate::OGT, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                index.find_type_information("BOOL").unwrap(),
            ),

            Operator::LessOrEqual => (
                builder
                    .build_float_compare(FloatPredicate::OLE, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                index.find_type_information("BOOL").unwrap(),
            ),

            Operator::GreaterOrEqual => (
                builder
                    .build_float_compare(FloatPredicate::OGE, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                index.find_type_information("BOOL").unwrap(),
            ),

            _ => unimplemented!(),
        };
        (data_type, value)
    }
