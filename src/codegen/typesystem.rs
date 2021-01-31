use crate::index::Index;

/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use super::{CodeGen, DataTypeInformation, LValue, TypeAndValue};
use inkwell::{builder::Builder, context::Context, types::{BasicType, BasicTypeEnum, IntType}};
use inkwell::values::{BasicValue, BasicValueEnum, IntValue};

impl<'ctx> CodeGen<'ctx> {
    pub fn initialize_type_system(&mut self) {
        let c = self.context;

        self.index.register_type("__VOID".to_string());
        self.index.associate_type(
            "__VOID",
            DataTypeInformation::Void {
            },
        );

        self.index.register_type("BOOL".to_string());
        self.index.associate_type(
            "BOOL",
            DataTypeInformation::Integer {
                signed: true,
                size: 1,
                generated_type: c.bool_type().as_basic_type_enum(),
            },
        );

        self.index.register_type("BYTE".to_string());
        self.index.associate_type(
            "BYTE",
            DataTypeInformation::Integer {
                signed: false,
                size: 8,
                generated_type: c.i8_type().as_basic_type_enum(),
            },
        );

        self.index.register_type("SINT".to_string());
        self.index.associate_type(
            "SINT",
            DataTypeInformation::Integer {
                signed: true,
                size: 8,
                generated_type: c.i8_type().as_basic_type_enum(),
            },
        );

        self.index.register_type("USINT".to_string());
        self.index.associate_type(
            "USINT",
            DataTypeInformation::Integer {
                signed: false,
                size: 8,
                generated_type: c.i8_type().as_basic_type_enum(),
            },
        );

        self.index.register_type("WORD".to_string());
        self.index.associate_type(
            "WORD",
            DataTypeInformation::Integer {
                signed: false,
                size: 16,
                generated_type: c.i16_type().as_basic_type_enum(),
            },
        );

        self.index.register_type("INT".to_string());
        self.index.associate_type(
            "INT",
            DataTypeInformation::Integer {
                signed: true,
                size: 16,
                generated_type: c.i16_type().as_basic_type_enum(),
            },
        );

        self.index.register_type("UINT".to_string());
        self.index.associate_type(
            "UINT",
            DataTypeInformation::Integer {
                signed: false,
                size: 16,
                generated_type: c.i16_type().as_basic_type_enum(),
            },
        );

        self.index.register_type("DWORD".to_string());
        self.index.associate_type(
            "DWORD",
            DataTypeInformation::Integer {
                signed: false,
                size: 32,
                generated_type: c.i32_type().as_basic_type_enum(),
            },
        );

        self.index.register_type("DINT".to_string());
        self.index.associate_type(
            "DINT",
            DataTypeInformation::Integer {
                signed: true,
                size: 32,
                generated_type: c.i32_type().as_basic_type_enum(),
            },
        );

        self.index.register_type("UDINT".to_string());
        self.index.associate_type(
            "UDINT",
            DataTypeInformation::Integer {
                signed: false,
                size: 32,
                generated_type: c.i32_type().as_basic_type_enum(),
            },
        );

        self.index.register_type("LWORD".to_string());
        self.index.associate_type(
            "LWORD",
            DataTypeInformation::Integer {
                signed: false,
                size: 64,
                generated_type: c.i64_type().as_basic_type_enum(),
            },
        );

        self.index.register_type("LINT".to_string());
        self.index.associate_type(
            "LINT",
            DataTypeInformation::Integer {
                signed: true,
                size: 64,
                generated_type: c.i64_type().as_basic_type_enum(),
            },
        );

        self.index.register_type("ULINT".to_string());
        self.index.associate_type(
            "ULINT",
            DataTypeInformation::Integer {
                signed: false,
                size: 64,
                generated_type: c.i64_type().as_basic_type_enum(),
            },
        );

        self.index.register_type("REAL".to_string());
        self.index.associate_type(
            "REAL",
            DataTypeInformation::Float {
                size: 32,
                generated_type: c.f32_type().as_basic_type_enum(),
            },
        );

        self.index.register_type("LREAL".to_string());
        self.index.associate_type(
            "LREAL",
            DataTypeInformation::Float {
                size: 64,
                generated_type: c.f64_type().as_basic_type_enum(),
            },
        );
        self.index.register_type("STRING".to_string());
        self.index.associate_type(
            "STRING",
            DataTypeInformation::String {
                size: 81,
                generated_type: c.i8_type().array_type(81).as_basic_type_enum(),
            },
        );
    }

    

        fn is_same_type_nature(
        &self,
        ltype: &DataTypeInformation<'ctx>,
        rtype: &DataTypeInformation<'ctx>,
    ) -> bool {
        ltype.is_int() == rtype.is_int()
    }

    

    pub fn get_bool_type_information(&self) -> DataTypeInformation<'ctx> {
        self.index.find_type_information("BOOL").unwrap()
    }
}
pub fn new_string_information<'ctx>(
        context: &'ctx Context,    
    len : u32
    ) -> DataTypeInformation<'ctx> {
        DataTypeInformation::String {
            size: len + 1,
            generated_type: context.i8_type().array_type(len+1).as_basic_type_enum(),        
        }
    }

pub fn get_default_for<'ctx>(basic_type : BasicTypeEnum<'ctx>) -> BasicValueEnum<'ctx> {
        match basic_type{
            BasicTypeEnum::ArrayType(t) => t.const_zero().as_basic_value_enum(),
            BasicTypeEnum::FloatType(t) => t.const_zero().as_basic_value_enum(),
            BasicTypeEnum::IntType(t) => t.const_zero().as_basic_value_enum(),
            BasicTypeEnum::PointerType(t) => t.const_zero().as_basic_value_enum(),
            BasicTypeEnum::StructType(t) => t.const_zero().as_basic_value_enum(),
            BasicTypeEnum::VectorType(t) => t.const_zero().as_basic_value_enum(), 
        }
    }

    pub fn promote_if_needed<'a>(
        builder: &Builder<'a>,
        lvalue: &TypeAndValue<'a>,
        rvalue: &TypeAndValue<'a>,
        index: &Index<'a>
    ) -> (
        DataTypeInformation<'a>,
        BasicValueEnum<'a>,
        BasicValueEnum<'a>,
    ) {
        let (ltype, lvalue) = lvalue;
        let (rtype, rvalue) = rvalue;

        let ltype_llvm = ltype.get_type();
        let rtype_llvm = rtype.get_type();

        if ltype.is_numerical() && rtype.is_numerical() {
            if ltype_llvm == rtype_llvm {
                (ltype.clone(), *lvalue, *rvalue)
            } else {
                let target_type = get_bigger_type(
                    &get_bigger_type(ltype, rtype, index),
                    &index.find_type_information("DINT").unwrap(),
                    index
                );

                let promoted_lvalue = promote_value_if_needed(builder, *lvalue, ltype, &target_type);
                let promoted_rvalue = promote_value_if_needed(builder, *rvalue, rtype, &target_type);

                return (target_type, promoted_lvalue, promoted_rvalue);
            }
        } else {
            panic!("Binary operations need numerical types")
        }
    }

pub fn get_bigger_type<'a>(
        ltype: &DataTypeInformation<'a>,
        rtype: &DataTypeInformation<'a>,
        index: &Index<'a>
    ) -> DataTypeInformation<'a> {
        let bigger_type = if is_same_type_nature(&ltype, &rtype) {
            if get_rank(&ltype) < get_rank(&rtype) {
                rtype.clone()
            } else {
                ltype.clone()
            }
        } else {
            let real_type = index.find_type_information("REAL").unwrap();
            let real_size = real_type.get_size();
            if ltype.get_size() > real_size || rtype.get_size() > real_size {
                index.find_type_information("LREAL").unwrap()
            } else {
                real_type
            }
        };
        bigger_type
    }

    fn get_rank(type_information: &DataTypeInformation) -> u32 {
        match type_information {
            DataTypeInformation::Integer { signed, size, .. } => {
                if *signed {
                    *size + 1
                } else {
                    *size
                }
            }
            DataTypeInformation::Float { size, .. } => size + 1000,
            _ => unreachable!(),
        }
    }

    fn is_same_type_nature(
        ltype: &DataTypeInformation,
        rtype: &DataTypeInformation,
    ) -> bool {
        (ltype.is_int() && ltype.is_int() == rtype.is_int())
        || (ltype.is_float() && ltype.is_float() == rtype.is_float())
    }

fn promote_value_if_needed<'ctx>(
        builder: &Builder<'ctx>,
        lvalue: BasicValueEnum<'ctx>,
        ltype: &DataTypeInformation<'ctx>,
        target_type: &DataTypeInformation<'ctx>,
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
                generated_type,
                ..
            } => {
                // INT --> INT
                let int_value = lvalue.into_int_value();
                if int_value.get_type().get_bit_width() < *target_size {
                    create_llvm_extend_int_value(builder, int_value, ltype, generated_type.into_int_type())
                        .as_basic_value_enum()
                } else {
                    lvalue
                }
            }
            DataTypeInformation::Float {
                size: target_size,
                generated_type: target_generated_type,
            } => {
                if let DataTypeInformation::Integer { signed, .. } = ltype {
                    // INT --> FLOAT
                    let int_value = lvalue.into_int_value();
                    if *signed {
                        builder
                            .build_signed_int_to_float(
                                int_value,
                                target_generated_type.into_float_type(),
                                "",
                            )
                            .into()
                    } else {
                        builder
                            .build_unsigned_int_to_float(
                                int_value,
                                target_generated_type.into_float_type(),
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
                                    target_generated_type.into_float_type(),
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
                size: _,
                generated_type: _,
            } => builder
                .build_int_s_extend_or_bit_cast(lvalue, target_type, ""),
            DataTypeInformation::Integer {
                signed: false,
                size: _,
                generated_type: _,
            } => builder
                .build_int_z_extend_or_bit_cast(lvalue, target_type, ""),
            _ => unreachable!(),
        }
    }

    pub fn cast_if_needed<'ctx>(
        builder: &Builder<'ctx>,
        target_type: &DataTypeInformation<'ctx>,
        value: BasicValueEnum<'ctx>,
        value_type: &DataTypeInformation<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String>{
       match target_type {
            DataTypeInformation::Integer {
                signed,
                size: lsize,
                generated_type,
            } => {
                match value_type {
                    DataTypeInformation::Integer { size: rsize, .. } => {
                        if lsize < rsize {
                            //Truncate
                            Ok(
                                builder
                                    .build_int_truncate_or_bit_cast(
                                        value.into_int_value(),
                                        generated_type.into_int_type(),
                                        "",
                                    )
                                    .into(),
                            )
                        } else {
                            //Expand
                            Ok(
                                promote_value_if_needed(builder,value, value_type, &target_type)
                                    .into(),
                            )
                        }
                    }
                    DataTypeInformation::Float {
                        size: _rsize,
                        generated_type: _,
                    } => {
                        if *signed {
                            Ok(
                                builder
                                    .build_float_to_signed_int(
                                        value.into_float_value(),
                                        generated_type.into_int_type(),
                                        "",
                                    )
                                    .into(),
                            )
                        } else {
                            Ok(
                                builder
                                    .build_float_to_unsigned_int(
                                        value.into_float_value(),
                                        generated_type.into_int_type(),
                                        "",
                                    )
                                    .into(),
                            )
                        }
                    }
                    _ => Err(format!("cannot cast from {:?} to {:?}", value_type, target_type)),
                }
            }
            DataTypeInformation::Float { generated_type, size : lsize, .. } => match value_type {
                DataTypeInformation::Integer { signed, .. } => {
                    if *signed {
                        Ok(
                            builder
                                .build_signed_int_to_float(
                                    value.into_int_value(),
                                    generated_type.into_float_type(),
                                    "",
                                )
                                .into(),
                        )
                    } else {
                        Ok(
                            builder
                                .build_unsigned_int_to_float(
                                    value.into_int_value(),
                                    generated_type.into_float_type(),
                                    "",
                                )
                                .into(),
                        )
                    }
                }
                DataTypeInformation::Float {size : rsize, ..} => {
                    if lsize < rsize {
                        Ok(builder.build_float_trunc(
                            value.into_float_value(),
                            generated_type.into_float_type(),
                            ""
                        ).into())
                    } else {
                        Ok(promote_value_if_needed(builder, value, value_type, &target_type))
                    }
                }
                _ => Err(format!("cannot cast from {:?} to {:?}", value_type, target_type)),
            },
            DataTypeInformation::String {size, ..} => match value_type {
                DataTypeInformation::String {size: value_size, ..} => {
                    if size < value_size {
                        //if we are on a vector replace it

                    Err(format!("cannot cast from {:?} to {:?}", value_type, target_type))
                        /*if value.is_vector_value() {
                            let vec_value = value.into_vector_value();
                            let string_value = vec_value.get_string_constant().to_bytes();
                            let new_value= &string_value[0..(*size -1) as usize];

                            let (_,value) = self.generate_literal_string(new_value);
                            value
                        }
                        else {
                            None
                        }*/
                    } else {
                        Ok(value)
                    }
                },
                _ => Err(format!("cannot cast from {:?} to {:?}", value_type, target_type)),
            } ,
            _ => Ok(value),
        }
    }

