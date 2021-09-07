// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::{
    ast::SourceRange,
    codegen::{TypeAndPointer, TypeAndValue},
    compile_error::CompileError,
    index::Index,
    typesystem::{self, VOID_TYPE},
};
use inkwell::{
    builder::Builder,
    context::Context,
    module::{Linkage, Module},
    types::{BasicTypeEnum, StringRadix},
    values::{BasicValue, BasicValueEnum, GlobalValue, IntValue, PointerValue},
    AddressSpace,
};

/// Holds dependencies required to generate IR-code
pub struct Llvm<'a> {
    pub context: &'a Context,
    pub builder: Builder<'a>,
}

impl<'a> Llvm<'a> {
    /// constructs a new LLVM struct
    pub fn new(context: &'a Context, builder: Builder<'a>) -> Llvm<'a> {
        Llvm { context, builder }
    }

    /// generates a global variable with the given name, datatype and optional initial value
    /// into the given module
    ///
    /// - `module` the compilation module to add the variable
    /// - `name` the name of the global variable
    /// - `data_type` the variable's datatype
    /// - `initial_value` an optional initial value of the global variable    
    pub fn create_global_variable(
        &self,
        module: &Module<'a>,
        name: &str,
        data_type: BasicTypeEnum<'a>,
        initial_value: Option<BasicValueEnum<'a>>,
    ) -> GlobalValue<'a> {
        let global = module.add_global(data_type, Some(AddressSpace::Generic), name);

        if let Some(initializer) = initial_value {
            let v = &initializer as &dyn BasicValue;
            global.set_initializer(v);
        } else {
            Self::set_const_zero_initializer(&global, data_type);
        }
        global.set_thread_local_mode(None);
        global.set_linkage(Linkage::External);
        global
    }

    /// creates a local variable at the builder's location
    ///
    /// - `name` the name of the local variable
    /// - `data_type` the variable's datatype
    pub fn create_local_variable(
        &self,
        name: &str,
        data_type: &BasicTypeEnum<'a>,
    ) -> PointerValue<'a> {
        self.builder.build_alloca(*data_type, name)
    }

    /// sets a const-zero initializer for the given global_value according to the given type
    /// sets a const_zero initializer if the given variable_type is either an int_type or a struct_type
    ///
    /// - `global_value` the value to set the initializer on
    /// - `variable_type` the data_type of the variable to initialize
    fn set_const_zero_initializer<'ctx>(
        global_value: &GlobalValue<'ctx>,
        variable_type: BasicTypeEnum<'ctx>,
    ) {
        if variable_type.is_int_type() {
            global_value.set_initializer(&variable_type.into_int_type().const_zero());
        } else if variable_type.is_struct_type() {
            global_value.set_initializer(&variable_type.into_struct_type().const_zero());
        }
    }

    /// loads a value from the given array into a variable with the given name
    ///
    /// - `pointer_to_array_instance` a pointer to an array to load the value from
    /// - `access_sequence` a sequence of IntValue's used to access the array. For multi-dimensional arrays
    ///    you may provide multiple accessors
    /// - `name` the name of the resulting variable
    pub fn load_array_element(
        &self,
        pointer_to_array_instance: PointerValue<'a>,
        accessor_sequence: &[IntValue<'a>],
        name: &str,
    ) -> Result<PointerValue<'a>, CompileError> {
        unsafe {
            Ok(self
                .builder
                .build_in_bounds_gep(pointer_to_array_instance, accessor_sequence, name))
        }
    }

    /// creates a pointervalue that points to a member of a struct
    ///
    /// - `pointer_to_struct_instance` a pointer to the struct
    /// - `member_index` the index of the member we want a pointer to
    /// - `name` the name of the temporary variable
    /// - `offset` the location in case of a CompileError
    pub fn get_member_pointer_from_struct(
        &self,
        pointer_to_struct_instance: PointerValue<'a>,
        member_index: u32,
        name: &str,
        offset: &SourceRange,
    ) -> Result<PointerValue<'a>, CompileError> {
        self.builder
            .build_struct_gep(pointer_to_struct_instance, member_index, name)
            .map_err(|_| {
                CompileError::codegen_error(
                    format!("Cannot generate qualified reference for {:}", name),
                    offset.clone(),
                )
            })
    }

    /// loads the value behind the given pointer
    ///
    /// - `lvalue` the pointer and it's datatype
    /// - `name` the name of the temporary variable
    pub fn load_pointer(&self, lvalue: &TypeAndPointer<'a, '_>, name: &str) -> TypeAndValue<'a> {
        (
            lvalue.get_type_information().clone(),
            self.builder.build_load(lvalue.ptr_value, name),
        )
    }

    /// creates a placeholder datatype for a struct with the given name
    ///
    /// returns an opaque_struct with the given name
    pub fn create_struct_stub(&self, name: &str) -> inkwell::types::StructType<'a> {
        self.context.opaque_struct_type(name)
    }

    /// returns the i32_type
    pub fn i32_type(&self) -> inkwell::types::IntType<'a> {
        self.context.i32_type()
    }

    /// returns the bool_type
    pub fn bool_type(&self) -> inkwell::types::IntType<'a> {
        self.context.bool_type()
    }

    /// create a constant bool with the given value
    ///
    /// - `index` the index to obtain the datatypeinformation for BOOL
    /// - `value` the value of the constant bool value
    pub fn create_const_bool(
        &self,
        index: &Index,
        value: bool,
    ) -> Result<TypeAndValue<'a>, CompileError> {
        let itype = self.context.bool_type();
        let value = itype.const_int(value as u64, false);

        let data_type = index.get_type_information("BOOL")?;
        Ok((data_type, BasicValueEnum::IntValue(value)))
    }

    /// create a constant int or float with the given value and the given type
    ///
    /// - `llvm_index` the index to obtain the BasicTypeEnum for the given target_type
    /// - `expected_type` the target int_type
    /// - `value` the value of the constant int value
    pub fn create_const_numeric(
        &self,
        target_type: &BasicTypeEnum<'a>,
        value: &str,
    ) -> Result<BasicValueEnum<'a>, CompileError> {
        match target_type {
            BasicTypeEnum::IntType { 0: int_type } => {
                let value = int_type.const_int_from_string(value, StringRadix::Decimal);
                Ok(BasicValueEnum::IntValue(value.unwrap()))
            }
            BasicTypeEnum::FloatType { 0: float_type } => {
                let value = float_type.const_float_from_string(value);
                Ok(BasicValueEnum::FloatValue(value))
            }
            _ => Err(CompileError::codegen_error(
                "expected numeric type".into(),
                SourceRange::undefined(),
            )),
        }
    }

    /// create a null pointer
    pub fn create_null_ptr(&self) -> Result<TypeAndValue<'a>, CompileError> {
        let itype = self.context.i32_type().ptr_type(AddressSpace::Generic);
        let value = itype.const_null();

        let data_type = typesystem::DataTypeInformation::Pointer {
            name: "VOIDPtr".into(),
            inner_type_name: VOID_TYPE.into(),
            auto_deref: false,
        };
        Ok((data_type, value.into()))
    }

    /// create a constant utf8 string-value with the given value
    ///
    /// - `value` the value of the constant string value
    pub fn create_const_utf8_string(&self, value: &str) -> Result<TypeAndValue<'a>, CompileError> {
        self.create_llvm_const_vec_string(value.as_bytes())
    }

    /// create a constant utf16 string-value with the given value
    ///
    /// - `value` the value of the constant string value
    pub fn create_const_utf16_string(&self, value: &str) -> Result<TypeAndValue<'a>, CompileError> {
        let mut utf16_chars: Vec<u16> = value.encode_utf16().collect();
        //it only contains a single NUL-terminator-byte so we add a second one
        utf16_chars.push(0);
        self.create_llvm_const_utf16_vec_string(utf16_chars.as_slice())
    }

    /// create a constant utf16 string-value with the given value
    ///
    /// - `value` the value of the constant string value
    pub fn create_llvm_const_utf16_vec_string(
        &self,
        value: &[u16],
    ) -> Result<TypeAndValue<'a>, CompileError> {
        let mut bytes = Vec::with_capacity(value.len() * 2);
        value.iter().for_each(|it| {
            let ordered_bytes = it.to_le_bytes(); //todo make this a compiler-setting
            bytes.push(ordered_bytes[0]);
            bytes.push(ordered_bytes[1]);
        });

        let exp_value = self.context.const_string(bytes.as_slice(), false);
        Ok((
            typesystem::new_wide_string_information(value.len() as u32),
            BasicValueEnum::VectorValue(exp_value),
        ))
    }

    /// create a constant utf8 string-value with the given value
    ///
    /// - `value` the value of the constant string value
    pub fn create_llvm_const_vec_string(
        &self,
        value: &[u8],
    ) -> Result<TypeAndValue<'a>, CompileError> {
        let exp_value = self.context.const_string(value, true);
        Ok((
            typesystem::new_string_information(value.len() as u32),
            BasicValueEnum::VectorValue(exp_value),
        ))
    }
}
