/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use inkwell::{types::{BasicTypeEnum, StructType}, values::{BasicValueEnum}};
use crate::{ast::{Variable}, codegen::typesystem, compile_error::CompileError, index::{Index}};
use super::{expression_generator::{ExpressionCodeGenerator}, llvm::LLVM};

/// object that offers convinient operations to create struct types and instances
pub struct StructGenerator<'a, 'b> {
    llvm: &'b LLVM<'a>,
    global_index: &'b mut Index<'a>,
}

///
/// a touple (name, data_type, initializer) describing the declaration of a variable.
///
type VariableDeclarationInformation<'a> = (String, BasicTypeEnum<'a>, Option<BasicValueEnum<'a>>);
type StructTypeAndValue<'a> = (StructType<'a>, BasicValueEnum<'a>);

impl<'a, 'b> StructGenerator<'a, 'b> {

    /// creates a new StructGenerator
    pub fn new(llvm: &'b LLVM<'a>, global_index: &'b mut Index<'a> ) -> StructGenerator<'a, 'b> {
        StructGenerator{
            llvm,
            global_index,
        }       
    }

    /// generates a new StructType with the given members
    ///
    /// - `member_variables` the member variables in the order of their declaration
    /// - `name` the name of the StructType
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
        
        //vec(member_name, initial_value)
        let struct_fields_values = members.iter()
                .map(|(name,basic_type, initializer)| 
                    initializer.map(|it| (name, it))
                    .unwrap_or_else(|| (name,typesystem::get_default_for(basic_type.clone()))))
                .collect::<Vec<(&String, BasicValueEnum)>>();

        for (member_name, initial_value) in &struct_fields_values {
            self.global_index.associate_member_initial_value(name, member_name, initial_value.clone());
        }

        let initial_value = struct_type.const_named_struct(
            struct_fields_values.iter().map(|(_, it)| *it).collect::<Vec<BasicValueEnum<'a>>>().as_slice());
        
        Ok((struct_type, initial_value.into()))
    }

    /// creates all declaration information for the given variable
    ///
    /// returns a tuple of the variable's name, its DataType and it's optional initial Value
    fn create_llvm_variable_declaration_elements(&self,
            variable: &Variable,
        )->Result<VariableDeclarationInformation<'a>, CompileError> {
            
            let type_name = variable.data_type.get_name().unwrap(); //TODO
            let type_index_entry = self.global_index.get_type(type_name)?;
                                    //&variable.data_type.get_name().ok_or_else(|| error_type_not_associated(type_name, &variable.location))?;

            let variable_type = type_index_entry.get_type_information().unwrap();
            let initializer = match &variable.initializer {
                Some(statement) => {
                    let exp_gen = ExpressionCodeGenerator::new_context_free(self.llvm, self.global_index, Some(variable_type.clone()));
                    exp_gen.generate_expression(statement)
                        .map(|(_, value)| Some(value))?
                }
                None => self.global_index.get_type_initial_value(type_name),
            };

            Ok((variable.name.to_string(), variable_type.get_type(), initializer))
        }


    }

/// returns the instance-name of a pou-struct
pub fn get_pou_instance_variable_name(pou_name: &str) -> String {
    format!("{}_instance", pou_name)
}