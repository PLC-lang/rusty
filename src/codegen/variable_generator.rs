/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use inkwell::{
    builder::Builder,
    context::Context,
    module::{Linkage, Module},
    types::BasicTypeEnum,
    values::{BasicValue, BasicValueEnum, GlobalValue, PointerValue},
    AddressSpace,
};

use crate::{
    ast::Variable,
    compile_error::CompileError,
    index::{Index},
};

use super::{statement_generator::StatementCodeGenerator, TypeAndPointer, TypeAndValue};

pub fn generate_global_variable<'ctx>(
    module: &Module<'ctx>,
    context: &'ctx Context,
    builder: &Builder<'ctx>,
    index: &mut Index<'ctx>,
    variable: &Variable,
) -> Result<GlobalValue<'ctx>, CompileError> {
    let type_name = variable.data_type.get_name().unwrap(); //TODO
    let variable_type_description = index.get_type(type_name)?;
    let variable_type = variable_type_description.get_type().unwrap();

    let initial_value = if let Some(initializer) = &variable.initializer {
        let statement_generator = StatementCodeGenerator::new(context, index, None);
        let (_, value) = statement_generator.generate_expression(&initializer, builder)?;
        //Todo cast if necessary
        Some(value)
    } else {
        None
    };
    let inital_value = initial_value.or(variable_type_description.get_initial_value());

    let global_variable =
        create_llvm_global_variable(module, &variable.name, variable_type, inital_value);

    index.associate_global_variable(&variable.name, global_variable.as_pointer_value());
    Ok(global_variable)
}

pub fn create_llvm_global_variable<'ctx>(
    module: &Module<'ctx>,
    name: &str,
    variable_type: BasicTypeEnum<'ctx>,
    initial_value: Option<BasicValueEnum<'ctx>>,
) -> GlobalValue<'ctx> {
    let result = module.add_global(variable_type, Some(AddressSpace::Generic), name);
    if let Some(initializer) = initial_value {
        let v = &initializer as &dyn BasicValue;
        result.set_initializer(v);
    } else {
        set_initializer_for_type(&result, variable_type);
    }
    result.set_thread_local_mode(None);
    result.set_linkage(Linkage::External);
    result
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

pub fn create_llvm_local_variable<'a>(
    builder: &Builder<'a>,
    name: &str,
    variable_type: &BasicTypeEnum<'a>,
) -> PointerValue<'a> {
    builder.build_alloca(*variable_type, name)
}

pub fn create_llvm_load_pointer<'a>(
    builder: &Builder<'a>,
    lvalue: &TypeAndPointer<'a, '_>,
    name: &str,
) -> Result<TypeAndValue<'a>, CompileError> {
    Ok((
        lvalue.get_type_information().clone(),
        builder.build_load(lvalue.ptr_value, name).into(),
    ))
}
