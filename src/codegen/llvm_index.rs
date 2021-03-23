use std::ops::RangeFull;

/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use indexmap::IndexMap;
use inkwell::values::{BasicValueEnum, FunctionValue, GlobalValue, PointerValue};
use inkwell::types::BasicTypeEnum;
use crate::compile_error::CompileError ;

/// Index view containing declared values for the current context
/// Parent Index is the a fallback lookup index for values not declared locally
#[derive(Debug, Clone)]
pub struct LLVMTypedIndex<'ink> {
    parent_index : Option<&'ink LLVMTypedIndex<'ink>>,
    type_associations : IndexMap<String, BasicTypeEnum<'ink>>,
    initial_value_associations : IndexMap<String, BasicValueEnum<'ink>>,
    loaded_variable_associations : IndexMap<String, PointerValue<'ink>>,
    implementations : IndexMap<String, FunctionValue<'ink>>,
}

impl<'ink> LLVMTypedIndex<'ink> {
    pub fn new() -> LLVMTypedIndex<'ink> {
        LLVMTypedIndex {
            parent_index : None,
            type_associations : IndexMap::new(),
            initial_value_associations : IndexMap::new(),
            loaded_variable_associations : IndexMap::new(),
            implementations : IndexMap::new(),
        }
    }

    pub fn create_child(parent : &'ink LLVMTypedIndex<'ink>) -> LLVMTypedIndex<'ink>{
        LLVMTypedIndex {
            parent_index : Some(parent),
            type_associations : IndexMap::new(),
            initial_value_associations : IndexMap::new(),
            loaded_variable_associations : IndexMap::new(),
            implementations : IndexMap::new(),
        }
    }

    //TODO : Try to use mut self 
    pub fn merge(&mut self, mut other : LLVMTypedIndex<'ink>) { //-> LLVMTypedIndex<'ctx> {

        for (name, assocication) in other.type_associations.drain(RangeFull) {
            self.type_associations.insert(name.into(),assocication);
        }
        for (name, assocication) in other.initial_value_associations.drain(RangeFull) {
            self.initial_value_associations.insert(name.into(),assocication);
        }
        for (name, assocication) in other.initial_value_associations.drain(RangeFull) {
            self.initial_value_associations.insert(name.into(),assocication);
        }
        for (name, assocication) in other.loaded_variable_associations.drain(RangeFull) {
            self.loaded_variable_associations.insert(name.into(),assocication);
        }
        for (name, implementation) in other.implementations.drain(RangeFull) {
            self.implementations.insert(name.into(),implementation);
        }
        // index
    }

    pub fn associate_type(&mut self, type_name : &str, target_type : BasicTypeEnum<'ink>) -> Result<(),CompileError>{
        self.type_associations.insert(type_name.into(), target_type); 
        Ok(())
    }

    pub fn associate_initial_value(&mut self, type_name : &str, initial_value : BasicValueEnum<'ink>) -> Result<(), CompileError> {
        self.initial_value_associations.insert(type_name.into(), initial_value); 
        Ok(())
    }

    pub fn associate_loded_local_variable(&mut self, container_name : &str, variable_name: &str, target_value: PointerValue<'ink>) -> Result<(), CompileError>{
        let qualified_name = format!("{}.{}", container_name, variable_name);
        self.loaded_variable_associations.insert(qualified_name, target_value); 
        Ok(())
    }

    pub fn find_associated_type(&self, type_name : &str) -> Option<BasicTypeEnum<'ink>>{
        self.type_associations.get(type_name).map(|it| *it).or_else(|| 
            self.parent_index.map(|it|it.find_associated_type(type_name)).flatten()
        )
    }

    pub fn get_associated_type(&self, type_name : &str) -> Result<BasicTypeEnum<'ink>, CompileError>{
        self.find_associated_type(type_name).ok_or_else(|| CompileError::unknown_type(type_name, 0..0))
    }

    pub fn find_associated_initial_value(&self, type_name : &str) -> Option<BasicValueEnum<'ink>> {
        self.initial_value_associations.get(type_name).map(|it| *it).or_else(|| 
            self.parent_index.map(|it|it.find_associated_initial_value(type_name)).flatten()
        )
    }

    pub fn associate_global(&mut self, variable_name: &str, global_variable : GlobalValue<'ink>) -> Result<(), CompileError> {
        self.initial_value_associations.insert(variable_name.into(), global_variable.as_pointer_value().into()); 
        Ok(())
    }

    pub fn associate_implementation(&mut self, callable_name: &str, function_value : FunctionValue<'ink>) -> Result<(), CompileError> {
        self.implementations.insert(callable_name.into(), function_value);
        Ok(())
    }

    pub fn find_associated_implementation(&self, callable_name : &str) -> Option<FunctionValue<'ink>> {
        self.implementations.get(callable_name).map(|it| *it).or_else(|| 
            self.parent_index.map(|it|it.find_associated_implementation(callable_name)).flatten()
        )
    }

    pub fn find_associated_variable_value(&self, qualified_name: &str) -> Option<BasicValueEnum<'ink>> {
        self.initial_value_associations.get(qualified_name).map(|it| *it).or_else(|| 
            self.parent_index.map(|it|it.find_associated_variable_value(qualified_name)).flatten()
        )

    }

    pub fn find_loaded_associated_variable_value(&self, qualified_name: &str) -> Option<PointerValue<'ink>> {
        let result = self.loaded_variable_associations.get(qualified_name).map(|it| *it).or_else(|| 
            self.parent_index.map(|it|it.find_loaded_associated_variable_value(qualified_name)).flatten()
        );

        //If nothing got associated, see if we have a global we could reuse
        result.or_else(||self.find_associated_variable_value(qualified_name).filter(|it|it.is_pointer_value()).map(BasicValueEnum::into_pointer_value))
    }
}
