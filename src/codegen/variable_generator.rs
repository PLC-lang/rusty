use inkwell::{AddressSpace, builder::Builder, module::{Linkage, Module}, types::BasicTypeEnum, values::{BasicValue, BasicValueEnum, GlobalValue, PointerValue}};

use crate::{ast::Variable, index::{DataTypeIndexEntry, Index}};

use super::{LValue, TypeAndValue};


pub fn generate_global_variable<'ctx>(
    module: &Module<'ctx>,
    index: &mut Index<'ctx>,
    variable: &Variable,
) -> Result<GlobalValue<'ctx>, String> {
   
    let type_name = variable.data_type.get_name().unwrap(); //TODO 
    let variable_type = index.find_type(type_name).unwrap().get_type().unwrap();
    let global_variable = create_llvm_global_variable(module, &variable.name, variable_type, None); //TODO

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

fn set_initializer_for_type<'ctx>(global_value: &GlobalValue<'ctx>, variable_type: BasicTypeEnum<'ctx>) {
    if variable_type.is_int_type() {
        global_value.set_initializer(&variable_type.into_int_type().const_zero());
    } else if variable_type.is_struct_type() {
        global_value.set_initializer(&variable_type.into_struct_type().const_zero());
    }
}

pub fn find_default_initializer_for<'ctx>(type_name: &str, index: &Index<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        index.find_type(type_name).and_then(DataTypeIndexEntry::get_initial_value)
    }

pub fn create_llvm_local_variable<'a>(builder: &Builder<'a>, name: &str, variable_type: &BasicTypeEnum<'a>) -> PointerValue<'a> {
    builder.build_alloca(*variable_type, name)
}

pub fn create_llvm_load_pointer<'a>(builder: &Builder<'a>, lvalue: &LValue<'a>, name : &str) -> Result<TypeAndValue<'a>, String> {
        Ok((
            lvalue.type_information.clone(),
            builder.build_load(lvalue.ptr_value, name).into(),
        ))
    }

//pub fn create_llvm_load_pointer<'a>(builder: &Builder<'a>, type_and_value: &TypeAndValue<'a>, name: &str) -> Result<TypeAndValue

