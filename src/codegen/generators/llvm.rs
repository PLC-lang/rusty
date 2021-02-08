use inkwell::{AddressSpace, builder::Builder, context::Context, module::{Linkage, Module}, types::{BasicType, BasicTypeEnum, StringRadix}, values::{BasicValue, BasicValueEnum, GlobalValue, IntValue, PointerValue}};

use crate::{ast::SourceRange, codegen::{TypeAndPointer, TypeAndValue, typesystem}, compile_error::CompileError, index::Index};


pub struct LLVM<'a> {
    pub context: &'a Context,
    pub builder: Builder<'a>,
}

impl<'a> LLVM<'a> {
    pub fn new(context: &'a Context, builder: Builder<'a>) -> LLVM<'a> {
        LLVM {
            context,
            builder,
        }
    }

    pub fn create_global_variable(
        &self,
        module: &Module<'a>,
        name: &str,
        data_type: BasicTypeEnum<'a>,
        initial_value: Option<BasicValueEnum<'a>>,
    ) -> GlobalValue<'a> {
        let global = module
            .add_global(data_type, Some(AddressSpace::Generic), name);

        if let Some(initializer) = initial_value {
            let v = &initializer as &dyn BasicValue;
            global.set_initializer(v);
        } else {
            Self::set_initializer_for_type(&global, data_type);
        }
        global.set_thread_local_mode(None);
        global.set_linkage(Linkage::External);
        global
    }

    pub fn allocate_local_variable(
        &self,
        name: &str,
        data_type: &BasicTypeEnum<'a>,
    ) -> PointerValue<'a> {
        self.builder.build_alloca(*data_type, name)
    }

    fn set_initializer_for_type<'ctx>(
        global_value: &GlobalValue<'ctx>,
        variable_type: BasicTypeEnum<'ctx>,
    ) {
        if variable_type.is_int_type() {
            global_value.set_initializer(&variable_type.into_int_type().const_zero());
        } else if variable_type.is_struct_type() {
            global_value.set_initializer(&variable_type.into_struct_type().const_zero());
        }
    }

    pub fn load_array_element(&self,
        pointer_to_array_instance: PointerValue<'a>,
        accessor_sequence: &[IntValue<'a>],
        name: &str
    ) -> Result<PointerValue<'a>, CompileError> {
        unsafe { 
            Ok(self.builder.build_in_bounds_gep(pointer_to_array_instance, accessor_sequence, name)) 
        }
    }

    pub fn load_member_from_struct(
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

    pub fn load_pointer(&self, lvalue: &TypeAndPointer<'a, '_>, name: &str) -> TypeAndValue<'a> {
        (
            lvalue.get_type_information().clone(),
            self.builder.build_load(lvalue.ptr_value, name).into(),
        )
    }

    pub fn create_struct_stub(&self, name: &str) -> inkwell::types::StructType<'a> {
        self.context.opaque_struct_type(name)
    }

    pub fn i32_type(&self) -> inkwell::types::IntType<'a> {
        self.context.i32_type()
    }

    pub fn create_const_bool(
        &self,
        index: &Index<'a>,
        value: bool,
    ) -> Result<TypeAndValue<'a>, CompileError> {
        let itype = self.context.bool_type();
        let value = itype.const_int(value as u64, false);

        let data_type = index.get_type_information("BOOL")?;
        Ok((data_type, BasicValueEnum::IntValue(value)))
    }

    pub fn create_const_int(
        &self,
        index: &Index<'a>,
        expected_type: &Option<BasicTypeEnum<'a>>,
        value: &str,
    ) -> Result<TypeAndValue<'a>, CompileError> {
        let i32_type = self.context.i32_type().as_basic_type_enum();
        let data_type = expected_type.unwrap_or(i32_type);

        if let BasicTypeEnum::IntType { 0: int_type } = data_type {
            let value = int_type.const_int_from_string(value, StringRadix::Decimal);
            let data_type = index.get_type_information("DINT")?;

            Ok((data_type, BasicValueEnum::IntValue(value.unwrap())))
        } else {
            panic!("error expected inttype");
        }
    }

    pub fn create_const_real(
        &self,
        index: &Index<'a>,
        expected_type: &Option<BasicTypeEnum<'a>>,
        value: &str,
    ) -> Result<TypeAndValue<'a>, CompileError> {
        let double_type = self.context.f32_type().as_basic_type_enum();
        let data_type = expected_type.unwrap_or(double_type);

        if let BasicTypeEnum::FloatType { 0: float_type } = data_type {
            let value = float_type.const_float_from_string(value);
            let data_type = index.get_type_information("REAL")?;
            Ok((data_type, BasicValueEnum::FloatValue(value)))
        } else {
            panic!("error expected floattype")
        }
    }

    pub fn create_const_string(
        &self,
        value: &str,
    ) -> Result<TypeAndValue<'a>, CompileError> {
        self.create_llvm_const_vec_string(value.as_bytes())
    }

    pub fn create_llvm_const_vec_string(
        &self,
        value: &[u8],
    ) -> Result<TypeAndValue<'a>, CompileError> {
        let exp_value = self.context.const_string(value, true);
        Ok((
            typesystem::new_string_information(self.context, value.len() as u32),
            BasicValueEnum::VectorValue(exp_value),
        ))
    }

}
