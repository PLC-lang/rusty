use crate::codegen::generators::data_type_generator::get_default_for;
use crate::codegen::llvm_index::LlvmTypedIndex;
use crate::codegen::CodegenError;
use crate::index::Index;
// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::typesystem::{CHAR_TYPE, WCHAR_TYPE};
use inkwell::types::{ArrayType, BasicType};
use inkwell::{
    builder::Builder,
    context::Context,
    module::{Linkage, Module},
    types::{BasicTypeEnum, StringRadix},
    values::{BasicValue, BasicValueEnum, GlobalValue, IntValue, PointerValue},
    AddressSpace,
};
use plc_ast::ast::AstNode;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;

use super::expression_generator::ExpressionCodeGenerator;
use super::ADDRESS_SPACE_GENERIC;

/// Holds dependencies required to generate IR-code
pub struct Llvm<'a> {
    pub context: &'a Context,
    pub builder: Builder<'a>,
}

pub trait GlobalValueExt {
    fn make_constant(self) -> Self;
    fn make_private(self) -> Self;
    fn make_external(self) -> Self;
    fn set_initial_value(self, initial_value: Option<BasicValueEnum>, data_type: BasicTypeEnum) -> Self;
}

impl GlobalValueExt for GlobalValue<'_> {
    fn make_constant(self) -> Self {
        self.set_constant(true);
        self.set_unnamed_addr(true);
        self
    }

    fn make_private(self) -> Self {
        self.set_linkage(Linkage::Private);
        self
    }

    fn make_external(self) -> Self {
        self.set_linkage(Linkage::External);
        self
    }

    fn set_initial_value(self, initial_value: Option<BasicValueEnum>, data_type: BasicTypeEnum) -> Self {
        if let Some(initializer) = initial_value {
            let v = &initializer as &dyn BasicValue;
            self.set_initializer(v);
        } else {
            Llvm::set_const_zero_initializer(&self, data_type);
        };
        self
    }
}

type Variable<'a> = (&'a str, &'a str, &'a SourceLocation);
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
    ) -> GlobalValue<'a> {
        let global = module.add_global(data_type, None, name);
        global.set_thread_local_mode(None);
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
    ) -> Result<PointerValue<'a>, CodegenError> {
        self.builder.build_alloca(*data_type, name).map_err(Into::into)
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
    ///   you may provide multiple accessors
    /// - `name` the name of the resulting variable
    pub fn load_array_element(
        &self,
        pointee: BasicTypeEnum<'a>,
        pointer_to_array_instance: PointerValue<'a>,
        accessor_sequence: &[IntValue<'a>],
        name: &str,
    ) -> Result<PointerValue<'a>, CodegenError> {
        unsafe {
            self.builder
                .build_in_bounds_gep(pointee, pointer_to_array_instance, accessor_sequence, name)
                .map_err(Into::into)
        }
    }

    /// creates a pointervalue that points to a member of a struct
    ///
    /// - `pointer_to_struct_instance` a pointer to the struct
    /// - `member_index` the index of the member we want a pointer to
    /// - `name` the name of the temporary variable
    /// - `offset` the location in case of a Diagnostic
    pub fn get_member_pointer_from_struct(
        &self,
        pointee: BasicTypeEnum<'a>,
        pointer_to_struct_instance: PointerValue<'a>,
        member_index: u32,
        name: &str,
    ) -> Result<PointerValue<'a>, CodegenError> {
        self.builder
            .build_struct_gep(pointee, pointer_to_struct_instance, member_index, name)
            .map_err(Into::into)
    }

    /// loads the value behind the given pointer
    ///
    /// - `lvalue` the pointer and it's datatype
    /// - `name` the name of the temporary variable
    pub fn load_pointer(
        &self,
        pointee_ty: BasicTypeEnum<'a>,
        lvalue: &PointerValue<'a>,
        name: &str,
    ) -> Result<BasicValueEnum<'a>, CodegenError> {
        self.builder.build_load(pointee_ty, lvalue.to_owned(), name).map_err(Into::into)
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

    /// returns the i64_type
    pub fn i64_type(&self) -> inkwell::types::IntType<'a> {
        self.context.i64_type()
    }

    /// create a constant bool with the given value
    ///
    /// - `index` the index to obtain the datatypeinformation for BOOL
    /// - `value` the value of the constant bool value
    pub fn create_const_bool(&self, value: bool) -> Result<BasicValueEnum<'a>, CodegenError> {
        let itype = self.context.bool_type();

        let value = if value { itype.const_all_ones() } else { itype.const_zero() };
        Ok(BasicValueEnum::IntValue(value))
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
        location: SourceLocation,
    ) -> Result<BasicValueEnum<'a>, CodegenError> {
        match target_type {
            BasicTypeEnum::IntType { 0: int_type } => int_type
                .const_int_from_string(value, StringRadix::Decimal)
                .ok_or_else(|| {
                    Diagnostic::codegen_error(format!("Cannot parse {value} as int"), location).into()
                })
                .map(BasicValueEnum::IntValue),
            BasicTypeEnum::FloatType { 0: float_type } => {
                let value = unsafe { float_type.const_float_from_string(value) };
                Ok(BasicValueEnum::FloatValue(value))
            }
            _ => Err(Diagnostic::codegen_error("expected numeric type", location).into()),
        }
    }

    /// create a null pointer
    pub fn create_null_ptr(&self) -> Result<BasicValueEnum<'a>, CodegenError> {
        let itype = self.context.ptr_type(AddressSpace::from(ADDRESS_SPACE_GENERIC));
        let value = itype.const_null();
        Ok(value.into())
    }

    /// create a constant utf8 string-value with the given value
    ///
    /// - `value` the value of the constant string value
    pub fn create_const_utf8_string(
        &self,
        value: &str,
        len: usize,
    ) -> Result<BasicValueEnum<'a>, CodegenError> {
        let mut utf8_chars = value.as_bytes()[..std::cmp::min(value.len(), len - 1)].to_vec();
        //fill the 0 terminators
        while utf8_chars.len() < len {
            utf8_chars.push(0);
        }
        self.create_llvm_const_vec_string(utf8_chars.as_slice())
    }

    /// create a constant utf16 string-value with the given value
    ///
    /// - `value` the value of the constant string value
    /// - `len` the len of the string, the literal will be right-padded with 0-bytes to match the length
    pub fn create_const_utf16_string(
        &self,
        value: &str,
        len: usize,
    ) -> Result<BasicValueEnum<'a>, CodegenError> {
        let mut utf16_chars: Vec<u16> = value.encode_utf16().collect();
        //fill the 0 terminators
        while utf16_chars.len() < len {
            utf16_chars.push(0);
        }
        self.create_llvm_const_utf16_vec_string(utf16_chars.as_slice())
    }

    /// create a constant utf16 string-value with the given value
    ///
    /// - `value` the value of the constant string value
    pub fn create_llvm_const_utf16_vec_string(
        &self,
        value: &[u16],
    ) -> Result<BasicValueEnum<'a>, CodegenError> {
        let values: Vec<IntValue> =
            value.iter().map(|it| self.context.i16_type().const_int(*it as u64, false)).collect();
        let vector = self.context.i16_type().const_array(&values);
        Ok(BasicValueEnum::ArrayValue(vector))
    }
    /// create a constant utf8 string-value with the given value
    ///
    /// - `value` the value of the constant string value
    pub fn create_llvm_const_vec_string(&self, value: &[u8]) -> Result<BasicValueEnum<'a>, CodegenError> {
        let values: Vec<IntValue> =
            value.iter().map(|it| self.context.i8_type().const_int(*it as u64, false)).collect();
        let vector = self.context.i8_type().const_array(&values);
        Ok(BasicValueEnum::ArrayValue(vector))
    }

    /// create a constant i8 character (IntValue) with the given value
    ///
    /// - `value` the value of the constant char value
    pub fn create_llvm_const_i8_char(
        &self,
        value: &str,
        location: &SourceLocation,
    ) -> Result<BasicValueEnum<'a>, CodegenError> {
        let arr = value.as_bytes();
        if let [first, ..] = arr {
            let value = self.context.i8_type().const_int(*first as u64, false);
            Ok(BasicValueEnum::IntValue(value))
        } else {
            Err(Diagnostic::cannot_generate_from_empty_literal(CHAR_TYPE, location).into())
        }
    }

    /// create a constant i16 character (IntValue) with the given value
    ///
    /// - `value` the value of the constant char value
    pub fn create_llvm_const_i16_char(
        &self,
        value: &str,
        location: &SourceLocation,
    ) -> Result<BasicValueEnum<'a>, CodegenError> {
        match value.encode_utf16().next() {
            Some(first) => {
                let value = self.context.i16_type().const_int(first as u64, false);
                Ok(BasicValueEnum::IntValue(value))
            }
            None => Err(Diagnostic::cannot_generate_from_empty_literal(WCHAR_TYPE, location).into()),
        }
    }

    pub fn get_array_type(llvm_type: BasicTypeEnum, size: u32) -> ArrayType {
        match llvm_type {
            BasicTypeEnum::ArrayType(_) => llvm_type.into_array_type().array_type(size),
            BasicTypeEnum::FloatType(_) => llvm_type.into_float_type().array_type(size),
            BasicTypeEnum::IntType(_) => llvm_type.into_int_type().array_type(size),
            BasicTypeEnum::PointerType(_) => llvm_type.into_pointer_type().array_type(size),
            BasicTypeEnum::StructType(_) => llvm_type.into_struct_type().array_type(size),
            BasicTypeEnum::VectorType(_) => llvm_type.into_vector_type().array_type(size),
            BasicTypeEnum::ScalableVectorType(_) => llvm_type.into_scalable_vector_type().array_type(size),
        }
    }

    /// initializes the variable represented by `variable` by storing into the given `variable_to_initialize` pointer using either
    /// the optional `initializer_statement` (hence code like: `variable : type := initializer_statement`), or determine the initial
    /// value with the help of the `variable`'s index entry by e.g. looking for a default value of the variable's type
    pub fn generate_variable_initializer(
        &self,
        llvm_index: &LlvmTypedIndex,
        index: &Index,
        variable: Variable,
        variable_to_initialize: PointerValue,
        initializer_statement: Option<&AstNode>,
        exp_gen: &ExpressionCodeGenerator,
    ) -> Result<(), CodegenError> {
        let (qualified_name, type_name, location) = variable;
        let variable_llvm_type = llvm_index.get_associated_type(type_name)?;

        let type_size = variable_llvm_type
            .size_of()
            .ok_or_else(|| Diagnostic::codegen_error("Couldn't determine type size", location));

        // initialize the variable with the initial_value
        let variable_data_type = index.get_effective_type_or_void_by_name(type_name);

        let v_type_info = variable_data_type.get_type_information();

        const DEFAULT_ALIGNMENT: u32 = 1;
        let (value, alignment) =
        // 1st try: see if there is a global variable with the right name - naming convention :-(
        if let Some(global_variable) =  llvm_index.find_global_value(&crate::index::get_initializer_name(qualified_name)) {
            (global_variable.as_basic_value_enum(), global_variable.get_alignment())
        // 2nd try: see if there is an initializer-statement
        } else if let Some(initializer) = initializer_statement {
            (exp_gen.generate_expression(initializer)?, DEFAULT_ALIGNMENT)
        // 3rd try: see if ther is a global variable with the variable's type name - naming convention :-(
        } else if let Some(global_variable) = llvm_index.find_global_value(&crate::index::get_initializer_name(variable_data_type.get_name())) {
            (global_variable.as_basic_value_enum(), global_variable.get_alignment())
        // 4th try, see if the datatype has a default initializer
        } else if let Some(initial_value) = llvm_index.find_associated_initial_value(type_name) {
            (initial_value, DEFAULT_ALIGNMENT)
        // no inital value defined + array type - so we use a 0 byte the memset the array to 0
        }else if v_type_info.is_array() || v_type_info.is_string() {
            (self.context.i8_type().const_zero().as_basic_value_enum(), DEFAULT_ALIGNMENT)
        // no initial value defined + no-array
        } else {
            (get_default_for(variable_llvm_type), DEFAULT_ALIGNMENT)
        };

        let is_aggregate_type = variable_data_type.is_aggregate_type();
        // initialize the variable with the initial_value
        if is_aggregate_type {
            // for arrays/structs, we prefere a memcpy, not a store operation
            // we assume that we got a global variable with the initial value that we can copy from
            if value.is_pointer_value() {
                // mem-copy from an global constant variable
                self.builder.build_memcpy(
                    variable_to_initialize,
                    std::cmp::max(1, alignment),
                    value.into_pointer_value(),
                    std::cmp::max(1, alignment),
                    type_size?,
                )?;
            } else if value.is_int_value() {
                // mem-set the value (usually 0) over the whole memory-area
                self.builder.build_memset(
                    variable_to_initialize,
                    std::cmp::max(1, alignment),
                    value.into_int_value(),
                    type_size?,
                )?;
            } else {
                Err(Diagnostic::codegen_error(
                    "initializing an array should be memcpy-able or memset-able",
                    location,
                ))?;
            };
        } else {
            self.builder.build_store(variable_to_initialize, value)?;
        }
        Ok(())
    }
}
