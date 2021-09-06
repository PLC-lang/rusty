// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::{ast::SourceRange, compile_error::CompileError};
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue, GlobalValue, PointerValue};
use std::collections::HashMap;

/// Index view containing declared values for the current context
/// Parent Index is the a fallback lookup index for values not declared locally
#[derive(Debug, Clone)]
pub struct LlvmTypedIndex<'ink> {
    parent_index: Option<&'ink LlvmTypedIndex<'ink>>,
    type_associations: HashMap<String, BasicTypeEnum<'ink>>,
    initial_value_associations: HashMap<String, BasicValueEnum<'ink>>,
    loaded_variable_associations: HashMap<String, PointerValue<'ink>>,
    implementations: HashMap<String, FunctionValue<'ink>>,
    constants: HashMap<String, BasicValueEnum<'ink>>,
}

impl<'ink> LlvmTypedIndex<'ink> {
    pub fn new() -> LlvmTypedIndex<'ink> {
        LlvmTypedIndex {
            parent_index: None,
            type_associations: HashMap::new(),
            initial_value_associations: HashMap::new(),
            loaded_variable_associations: HashMap::new(),
            implementations: HashMap::new(),
            constants: HashMap::new(),
        }
    }

    pub fn create_child(parent: &'ink LlvmTypedIndex<'ink>) -> LlvmTypedIndex<'ink> {
        LlvmTypedIndex {
            parent_index: Some(parent),
            type_associations: HashMap::new(),
            initial_value_associations: HashMap::new(),
            loaded_variable_associations: HashMap::new(),
            implementations: HashMap::new(),
            constants: HashMap::new(),
        }
    }

    pub fn merge(&mut self, mut other: LlvmTypedIndex<'ink>) {
        for (name, assocication) in other.type_associations.drain() {
            self.type_associations.insert(name, assocication);
        }
        for (name, assocication) in other.initial_value_associations.drain() {
            self.initial_value_associations.insert(name, assocication);
        }
        for (name, assocication) in other.initial_value_associations.drain() {
            self.initial_value_associations.insert(name, assocication);
        }
        for (name, assocication) in other.loaded_variable_associations.drain() {
            self.loaded_variable_associations.insert(name, assocication);
        }
        for (name, implementation) in other.implementations.drain() {
            self.implementations.insert(name, implementation);
        }
        self.constants.extend(other.constants);
    }

    pub fn associate_type(
        &mut self,
        type_name: &str,
        target_type: BasicTypeEnum<'ink>,
    ) -> Result<(), CompileError> {
        self.type_associations
            .insert(type_name.to_lowercase(), target_type);
        Ok(())
    }

    pub fn associate_initial_value(
        &mut self,
        type_name: &str,
        initial_value: BasicValueEnum<'ink>,
    ) -> Result<(), CompileError> {
        self.initial_value_associations
            .insert(type_name.to_lowercase(), initial_value);
        Ok(())
    }

    pub fn associate_loaded_local_variable(
        &mut self,
        container_name: &str,
        variable_name: &str,
        target_value: PointerValue<'ink>,
    ) -> Result<(), CompileError> {
        let qualified_name = format!("{}.{}", container_name, variable_name);
        self.loaded_variable_associations
            .insert(qualified_name.to_lowercase(), target_value);
        Ok(())
    }

    pub fn find_associated_type(&self, type_name: &str) -> Option<BasicTypeEnum<'ink>> {
        self.type_associations
            .get(&type_name.to_lowercase())
            .copied()
            .or_else(|| {
                self.parent_index
                    .map(|it| it.find_associated_type(type_name))
                    .flatten()
            })
    }

    pub fn get_associated_type(
        &self,
        type_name: &str,
    ) -> Result<BasicTypeEnum<'ink>, CompileError> {
        self.find_associated_type(type_name)
            .ok_or_else(|| CompileError::unknown_type(type_name, SourceRange::undefined()))
    }

    pub fn find_associated_initial_value(&self, type_name: &str) -> Option<BasicValueEnum<'ink>> {
        self.initial_value_associations
            .get(&type_name.to_lowercase())
            .copied()
            .or_else(|| {
                self.parent_index
                    .map(|it| it.find_associated_initial_value(type_name))
                    .flatten()
            })
    }

    pub fn associate_global(
        &mut self,
        variable_name: &str,
        global_variable: GlobalValue<'ink>,
    ) -> Result<(), CompileError> {
        self.initial_value_associations.insert(
            variable_name.to_lowercase(),
            global_variable.as_pointer_value().into(),
        );
        Ok(())
    }

    pub fn associate_implementation(
        &mut self,
        callable_name: &str,
        function_value: FunctionValue<'ink>,
    ) -> Result<(), CompileError> {
        self.implementations
            .insert(callable_name.to_lowercase(), function_value);
        Ok(())
    }

    pub fn find_associated_implementation(
        &self,
        callable_name: &str,
    ) -> Option<FunctionValue<'ink>> {
        self.implementations
            .get(&callable_name.to_lowercase())
            .copied()
            .or_else(|| {
                self.parent_index
                    .map(|it| it.find_associated_implementation(callable_name))
                    .flatten()
            })
    }

    pub fn find_associated_variable_value(
        &self,
        qualified_name: &str,
    ) -> Option<BasicValueEnum<'ink>> {
        self.initial_value_associations
            .get(&qualified_name.to_lowercase())
            .copied()
            .or_else(|| {
                self.parent_index
                    .map(|it| it.find_associated_variable_value(qualified_name))
                    .flatten()
            })
    }

    pub fn find_loaded_associated_variable_value(
        &self,
        qualified_name: &str,
    ) -> Option<PointerValue<'ink>> {
        let result = self
            .loaded_variable_associations
            .get(&qualified_name.to_lowercase())
            .copied()
            .or_else(|| {
                self.parent_index
                    .map(|it| it.find_loaded_associated_variable_value(qualified_name))
                    .flatten()
            });

        //If nothing got associated, see if we have a global we could reuse
        result.or_else(|| {
            self.find_associated_variable_value(qualified_name)
                .filter(|it| it.is_pointer_value())
                .map(BasicValueEnum::into_pointer_value)
        })
    }

    pub fn associate_constant(
        &mut self,
        qualified_name: &str,
        basic_value_enum: BasicValueEnum<'ink>,
    ) {
        self.constants
            .insert(qualified_name.into(), basic_value_enum);
    }

    pub fn find_constant_value(&self, qualified_name: &str) -> Option<BasicValueEnum<'ink>> {
        self.constants.get(qualified_name).copied()
    }
}
