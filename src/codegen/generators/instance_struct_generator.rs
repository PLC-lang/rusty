/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use inkwell::{types::{BasicTypeEnum, StructType}, values::{BasicValueEnum, PointerValue}};
use crate::{ast::{SourceRange, Variable}, codegen::typesystem, compile_error::CompileError, index::{Index}};
use super::{llvm::LLVM, statement_generator::StatementCodeGenerator };

pub struct InstanceStructGenerator<'a, 'b> {
    llvm: &'b LLVM<'a>,
    global_index: &'b Index<'a>,
}

///
/// a touple (name, data_type, initializer) describing the declaration of a variable.
///
type VariableDeclarationInformation<'a> = (String, BasicTypeEnum<'a>, Option<BasicValueEnum<'a>>);
type StructTypeAndValue<'a> = (StructType<'a>, BasicValueEnum<'a>);

impl<'a, 'b> InstanceStructGenerator<'a, 'b> {

    pub fn new(llvm: &'b LLVM<'a>, global_index: &'b Index<'a>) -> InstanceStructGenerator<'a, 'b> {
        InstanceStructGenerator{
            llvm,
            global_index,
        }       
    }

    pub fn generate_struct_type(
        &mut self,
        member_variables: &Vec<&Variable>,
        name: &str) -> Result<StructTypeAndValue<'a>, CompileError> {

        let struct_type_info = self.global_index.get_type(name)?;
        let struct_type = struct_type_info.get_type()
            .unwrap()
            .into_struct_type();

        let mut members = Vec::new();
        for member in member_variables {
            members.push(self.create_llvm_variable_declaration_elements(member)?);
        }

        let member_types: Vec<BasicTypeEnum> = members.iter().map(|(_, t, _)| *t).collect();
        struct_type.set_body(member_types.as_slice(), false);
        
        let struct_fields_values = members.iter()
                .map(|(_,basic_type, initializer)| 
                    initializer.unwrap_or_else(|| typesystem::get_default_for(basic_type.clone())))
                .collect::<Vec<BasicValueEnum>>();

        let initial_value = struct_type.const_named_struct(struct_fields_values.as_slice());
        Ok((struct_type, initial_value.into()))
    }

    fn create_llvm_variable_declaration_elements(&self,
            variable: &Variable
        )->Result<VariableDeclarationInformation<'a>, CompileError> {
            
            let type_name = variable.data_type.get_name().unwrap(); //TODO
            let type_index_entry = self.global_index.get_type(type_name)?;
                                    //&variable.data_type.get_name().ok_or_else(|| error_type_not_associated(type_name, &variable.location))?;

            let variable_type = type_index_entry.get_type_information().unwrap();
            let initializer = match &variable.initializer {
                Some(statement) => {
                    let statement_gen = StatementCodeGenerator::new_typed(
                            self.llvm, 
                            self.global_index, 
                            None, 
                            type_index_entry.get_type()
                                .ok_or_else(|| CompileError::no_type_associated(type_name, variable.location.clone()))?);

                    statement_gen.generate_expression(statement)
                        .map(|(_, value)| Some(value))?
                }
                None => 
                    type_index_entry.get_initial_value()
            };

            Ok((variable.name.to_string(), variable_type.get_type(), initializer))
        }


    pub fn allocate_struct_instance(&self, callable_name: &str, location: &SourceRange) -> Result<PointerValue<'a>, CompileError> {
        let instance_name = get_pou_instance_variable_name(callable_name);
        let function_type = self.global_index.get_type(callable_name)?
                                .get_type()
                                .ok_or_else(|| CompileError::no_type_associated(callable_name, location.clone()))?;

    Ok(self.llvm.allocate_local_variable(&instance_name, &function_type))
    }
}

pub fn get_pou_instance_variable_name(pou_name: &str) -> String {
    format!("{}_instance", pou_name)
}