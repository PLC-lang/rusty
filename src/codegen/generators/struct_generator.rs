// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::{expression_generator::ExpressionCodeGenerator, llvm::Llvm};
use crate::index::Index;
use crate::resolver::AnnotationMap;
use crate::{
    codegen::llvm_index::LlvmTypedIndex, compile_error::CompileError, index::VariableIndexEntry,
};
use inkwell::{types::BasicTypeEnum, values::BasicValueEnum};

/// object that offers convinient operations to create struct types and instances
pub struct StructGenerator<'a, 'b> {
    llvm: &'b Llvm<'a>,
    index: &'b Index,
    annotations: &'b AnnotationMap,
    llvm_index: &'b LlvmTypedIndex<'a>,
}

///
/// a touple (name, data_type, initializer) describing the declaration of a variable.
///
type VariableDeclarationInformation<'a> = (String, BasicTypeEnum<'a>, Option<BasicValueEnum<'a>>);

impl<'a, 'b> StructGenerator<'a, 'b> {
    /// creates a new StructGenerator
    pub fn new(
        llvm: &'b Llvm<'a>,
        index: &'b Index,
        annotations: &'b AnnotationMap,
        llvm_index: &'b LlvmTypedIndex<'a>,
    ) -> StructGenerator<'a, 'b> {
        StructGenerator {
            llvm,
            index,
            annotations,
            llvm_index,
        }
    }

    /// generates a new StructType with the given members
    ///
    /// - `member_variables` the member variables in the order of their declaration
    /// - `name` the name of the StructType
    pub fn generate_struct_type(
        &mut self,
        member_variables: &[&VariableIndexEntry],
        name: &str,
        //) -> Result<(StructValue<'a>, Vec<(String, BasicValueEnum<'a>)>), CompileError> {
    ) -> Result<(), CompileError> {
        let struct_type = self
            .llvm_index
            .get_associated_type(name)
            .map(BasicTypeEnum::into_struct_type)?;

        let mut members = Vec::new();
        for member in member_variables {
            members.push(self.create_llvm_variable_declaration_elements(member)?);
        }

        let member_types: Vec<BasicTypeEnum> = members.iter().map(|(_, t, _)| *t).collect();
        struct_type.set_body(member_types.as_slice(), false);
        Ok(())
        //vec(member_name, initial_value)
        // let struct_fields_values = members
        //     .iter()
        //     .map(|(name, basic_type, initializer)| {
        //         initializer
        //             .map(|it| (name, it))
        //             .unwrap_or_else(|| (name, get_default_for(*basic_type)))
        //     })
        //     .collect::<Vec<(&String, BasicValueEnum)>>();

        // let initial_value = struct_type.const_named_struct(
        //     struct_fields_values
        //         .iter()
        //         .map(|(_, it)| *it)
        //         .collect::<Vec<BasicValueEnum<'a>>>()
        //         .as_slice(),
        // );

        // let member_values = struct_fields_values
        //     .iter()
        //     .map(|(name, value)| (name.to_string(), *value))
        //     .collect();

        // Ok((initial_value.into(), member_values))
    }

    /// creates all declaration information for the given variable
    ///
    /// returns a tuple of the variable's name, its DataType and it's optional initial Value
    fn create_llvm_variable_declaration_elements(
        &self,
        variable: &VariableIndexEntry,
    ) -> Result<VariableDeclarationInformation<'a>, CompileError> {
        let type_name = variable.get_type_name();

        let initializer = match self
            .index
            .get_const_expressions()
            .maybe_get_constant_statement(&variable.initial_value)
        {
            Some(statement) => {
                //evalute the initializer to a value
                //TODO we should not resolve here, they should be resolved before!
                let evaluated_const =
                    crate::resolver::const_evaluator::evaluate(statement, None, self.index);
                match evaluated_const {
                    Ok(Some(initializer)) => {
                        //create the appropriate Literal AST-Statement
                        let ast_statement = initializer;
                        //generate the literal
                        let exp_gen = ExpressionCodeGenerator::new_context_free(
                            self.llvm,
                            self.index,
                            self.annotations,
                            self.llvm_index,
                        );

                        let result = exp_gen.generate_expression(&ast_statement).map(Some);

                        if let Err(CompileError::MissingFunctionError { .. }) = result {
                            return Err(CompileError::cannot_generate_initializer(
                                variable.get_qualified_name(),
                                statement.get_location(),
                            ));
                        } else {
                            result?
                        }
                    }
                    Err(err) => {
                        return Err(CompileError::codegen_error(
                            format!(
                                "Cannot generate literal initializer for '{:}': {:}",
                                variable.get_qualified_name(),
                                err
                            ),
                            statement.get_location(),
                        ));
                    }
                    Ok(None) => {
                        return Err(CompileError::cannot_generate_initializer(
                            variable.get_qualified_name(),
                            statement.get_location(),
                        ))
                    }
                }
            }
            None => self.llvm_index.find_associated_initial_value(type_name),
        };

        Ok((
            variable.get_name().to_string(),
            self.llvm_index.get_associated_type(type_name).unwrap(),
            initializer,
        ))
    }
}

/// returns the instance-name of a pou-struct
pub fn get_pou_instance_variable_name(pou_name: &str) -> String {
    format!("{}_instance", pou_name)
}

pub fn get_default_for(basic_type: BasicTypeEnum) -> BasicValueEnum {
    match basic_type {
        BasicTypeEnum::ArrayType(t) => t.const_zero().into(),
        BasicTypeEnum::FloatType(t) => t.const_zero().into(),
        BasicTypeEnum::IntType(t) => t.const_zero().into(),
        BasicTypeEnum::PointerType(t) => t.const_zero().into(),
        BasicTypeEnum::StructType(t) => t.const_zero().into(),
        BasicTypeEnum::VectorType(t) => t.const_zero().into(),
    }
}
