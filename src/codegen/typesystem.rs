/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use super::{CodeGen, DataTypeInformation};
use inkwell::types::{IntType,BasicType};
use inkwell::values::{BasicValue, BasicValueEnum, IntValue};
impl<'ctx> CodeGen<'ctx> {
    pub fn initialize_type_system(&mut self) {
        let c = self.context;

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

    pub fn new_string_information(
        &self,
        len : u32
    ) -> DataTypeInformation<'ctx> {
        DataTypeInformation::String {
            size: len + 1,
            generated_type: self.context.i8_type().array_type(len+1).as_basic_type_enum(),        
        }
    }

    pub fn cast_if_needed(
        &self,
        target_type: &DataTypeInformation<'ctx>,
        value: BasicValueEnum<'ctx>,
        value_type: &DataTypeInformation<'ctx>,
    ) -> Option<BasicValueEnum<'ctx>>{
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
                            Some(
                                self.builder
                                    .build_int_truncate_or_bit_cast(
                                        value.into_int_value(),
                                        generated_type.into_int_type(),
                                        "",
                                    )
                                    .into(),
                            )
                        } else {
                            //Expand
                            Some(
                                self.promote_value_if_needed(value, value_type, &target_type)
                                    .into(),
                            )
                        }
                    }
                    DataTypeInformation::Float {
                        size: _rsize,
                        generated_type: _,
                    } => {
                        if *signed {
                            Some(
                                self.builder
                                    .build_float_to_signed_int(
                                        value.into_float_value(),
                                        generated_type.into_int_type(),
                                        "",
                                    )
                                    .into(),
                            )
                        } else {
                            Some(
                                self.builder
                                    .build_float_to_unsigned_int(
                                        value.into_float_value(),
                                        generated_type.into_int_type(),
                                        "",
                                    )
                                    .into(),
                            )
                        }
                    }
                    _ => None,
                }
            }
            DataTypeInformation::Float { generated_type, .. } => match value_type {
                DataTypeInformation::Integer { signed, .. } => {
                    if *signed {
                        Some(
                            self.builder
                                .build_signed_int_to_float(
                                    value.into_int_value(),
                                    generated_type.into_float_type(),
                                    "",
                                )
                                .into(),
                        )
                    } else {
                        Some(
                            self.builder
                                .build_unsigned_int_to_float(
                                    value.into_int_value(),
                                    generated_type.into_float_type(),
                                    "",
                                )
                                .into(),
                        )
                    }
                }
                DataTypeInformation::Float { .. } => Some(value),
                _ => None,
            },
            DataTypeInformation::String {size, ..} => match value_type {
                DataTypeInformation::String {size: value_size, ..} => {
                    if size < value_size {
                        //if we are on a vector replace it
                        if value.is_vector_value() {
                            let vec_value = value.into_vector_value();
                            let string_value = vec_value.get_string_constant().to_bytes();
                            let new_value= &string_value[0..(*size -1) as usize];

                            let (_,value) = self.generate_literal_string(new_value);
                            value
                        }
                        else {
                            None
                        }
                    } else {
                        Some(value)
                    }
                },
                _ => None, 
            } ,
            _ => Some(value),
        }
    }

    fn promote_value_if_needed(
        &self,
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
                    self.extend_int_value(int_value, ltype, generated_type.into_int_type())
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
                        self.builder
                            .build_signed_int_to_float(
                                int_value,
                                target_generated_type.into_float_type(),
                                "",
                            )
                            .into()
                    } else {
                        self.builder
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
                            self.builder
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

    fn extend_int_value(
        &self,
        lvalue: IntValue<'ctx>,
        ltype: &DataTypeInformation,
        target_type: IntType<'ctx>,
    ) -> IntValue<'ctx> {
        match ltype {
            DataTypeInformation::Integer {
                signed: true,
                size: _,
                generated_type: _,
            } => self
                .builder
                .build_int_s_extend_or_bit_cast(lvalue, target_type, ""),
            DataTypeInformation::Integer {
                signed: false,
                size: _,
                generated_type: _,
            } => self
                .builder
                .build_int_z_extend_or_bit_cast(lvalue, target_type, ""),
            _ => unreachable!(),
        }
    }

    pub fn promote_if_needed(
        &self,
        lvalue: BasicValueEnum<'ctx>,
        ltype: &DataTypeInformation<'ctx>,
        rvalue: BasicValueEnum<'ctx>,
        rtype: &DataTypeInformation<'ctx>,
    ) -> (
        DataTypeInformation<'ctx>,
        BasicValueEnum<'ctx>,
        BasicValueEnum<'ctx>,
    ) {
        let ltype_llvm = ltype.get_type();
        let rtype_llvm = rtype.get_type();

        if ltype.is_numerical() && rtype.is_numerical() {
            if ltype_llvm == rtype_llvm {
                (ltype.clone(), lvalue, rvalue)
            } else {
                let target_type = self.get_bigger_type(
                    self.get_bigger_type(ltype.clone(), rtype.clone()),
                    self.index.find_type_information("DINT").unwrap(),
                );

                let promoted_lvalue = self.promote_value_if_needed(lvalue, ltype, &target_type);
                let promoted_rvalue = self.promote_value_if_needed(rvalue, rtype, &target_type);

                return (target_type, promoted_lvalue, promoted_rvalue);
            }
        } else {
            panic!("Binary operations need numerical types")
        }
    }

    fn is_same_type_nature(
        &self,
        ltype: &DataTypeInformation<'ctx>,
        rtype: &DataTypeInformation<'ctx>,
    ) -> bool {
        ltype.is_int() == rtype.is_int()
    }

    fn get_bigger_type(
        &self,
        ltype: DataTypeInformation<'ctx>,
        rtype: DataTypeInformation<'ctx>,
    ) -> DataTypeInformation<'ctx> {
        let bigger_type = if self.is_same_type_nature(&ltype, &rtype) {
            if self.get_rank(&ltype) < self.get_rank(&rtype) {
                rtype
            } else {
                ltype
            }
        } else {
            let real_type = self.index.find_type_information("REAL").unwrap();
            let real_size = real_type.get_size();
            if ltype.get_size() > real_size || rtype.get_size() > real_size {
                self.index.find_type_information("LREAL").unwrap()
            } else {
                real_type
            }
        };
        bigger_type
    }

    fn get_rank(&self, type_information: &DataTypeInformation) -> u32 {
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

    pub fn get_bool_type_information(&self) -> DataTypeInformation<'ctx> {
        self.index.find_type_information("BOOL").unwrap()
    }
}
