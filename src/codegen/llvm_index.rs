use crate::diagnostics::Diagnostic;
// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::ast::SourceRange;
use inkwell::debug_info::DIType;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue, GlobalValue, PointerValue};
use std::collections::HashMap;

/// Index view containing declared values for the current context
/// Parent Index is the a fallback lookup index for values not declared locally
#[derive(Debug, Clone, Default)]
pub struct LlvmTypedIndex<'ink> {
    parent_index: Option<&'ink LlvmTypedIndex<'ink>>,
    type_associations: HashMap<String, (BasicTypeEnum<'ink>, Option<DIType<'ink>>)>,
    pou_type_associations: HashMap<String, (BasicTypeEnum<'ink>, Option<DIType<'ink>>)>,
    global_values: HashMap<String, GlobalValue<'ink>>,
    initial_value_associations: HashMap<String, BasicValueEnum<'ink>>,
    loaded_variable_associations: HashMap<String, PointerValue<'ink>>,
    implementations: HashMap<String, FunctionValue<'ink>>,
    constants: HashMap<String, BasicValueEnum<'ink>>,
    utf08_literals: HashMap<String, GlobalValue<'ink>>,
    utf16_literals: HashMap<String, GlobalValue<'ink>>,
}

impl<'ink> LlvmTypedIndex<'ink> {
    pub fn create_child(parent: &'ink LlvmTypedIndex<'ink>) -> LlvmTypedIndex<'ink> {
        LlvmTypedIndex {
            parent_index: Some(parent),
            type_associations: HashMap::new(),
            pou_type_associations: HashMap::new(),
            global_values: HashMap::new(),
            initial_value_associations: HashMap::new(),
            loaded_variable_associations: HashMap::new(),
            implementations: HashMap::new(),
            constants: HashMap::new(),
            utf08_literals: HashMap::new(),
            utf16_literals: HashMap::new(),
        }
    }

    pub fn merge(&mut self, mut other: LlvmTypedIndex<'ink>) {
        for (name, assocication) in other.type_associations.drain() {
            self.type_associations.insert(name, assocication);
        }
        for (name, assocication) in other.pou_type_associations.drain() {
            self.pou_type_associations.insert(name, assocication);
        }
        for (name, assocication) in other.initial_value_associations.drain() {
            self.initial_value_associations.insert(name, assocication);
        }
        for (name, value) in other.global_values.drain() {
            self.global_values.insert(name, value);
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
        self.utf08_literals.extend(other.utf08_literals);
        self.utf16_literals.extend(other.utf16_literals);
    }

    pub fn associate_type(
        &mut self,
        type_name: &str,
        target_type: (BasicTypeEnum<'ink>, Option<DIType<'ink>>),
    ) -> Result<(), Diagnostic> {
        self.type_associations
            .insert(type_name.to_lowercase(), target_type);
        Ok(())
    }

    pub fn associate_pou_type(
        &mut self,
        type_name: &str,
        target_type: (BasicTypeEnum<'ink>, Option<DIType<'ink>>),
    ) -> Result<(), Diagnostic> {
        self.pou_type_associations
            .insert(type_name.to_lowercase(), target_type);
        Ok(())
    }

    pub fn associate_initial_value(
        &mut self,
        type_name: &str,
        initial_value: BasicValueEnum<'ink>,
    ) -> Result<(), Diagnostic> {
        self.initial_value_associations
            .insert(type_name.to_lowercase(), initial_value);
        Ok(())
    }

    pub fn associate_loaded_local_variable(
        &mut self,
        container_name: &str,
        variable_name: &str,
        target_value: PointerValue<'ink>,
    ) -> Result<(), Diagnostic> {
        let qualified_name = format!("{}.{}", container_name, variable_name);
        self.loaded_variable_associations
            .insert(qualified_name.to_lowercase(), target_value);
        Ok(())
    }

    pub fn find_global_value(&self, name: &str) -> Option<GlobalValue<'ink>> {
        self.global_values
            .get(&name.to_lowercase())
            .copied()
            .or_else(|| self.parent_index.and_then(|it| it.find_global_value(name)))
    }

    pub fn find_associated_type_and_debug(&self, type_name: &str) -> Option<(BasicTypeEnum<'ink>, Option<DIType<'ink>>)> {
        dbg!(self.type_associations
            .get(&type_name.to_lowercase())
            .copied()
            .or_else(|| {
                self.parent_index
                    .and_then(|it| it.find_associated_type_and_debug(type_name))
            })
            .or_else(|| self.find_associated_pou_type_and_debug(type_name)))
    }

    pub fn find_associated_type(&self, type_name: &str) -> Option<BasicTypeEnum<'ink>> {
        self.find_associated_type_and_debug(type_name).map(|(it, _) | it)
    }

    pub fn find_associated_debug_type(&self, type_name: &str) -> Option<DIType<'ink>> {
        self.find_associated_type_and_debug(type_name).and_then(|(_, it) | it)
    }

    pub fn find_associated_pou_type_and_debug(&self, type_name: &str) -> Option<(BasicTypeEnum<'ink>, Option<DIType<'ink>>)> {
        self.pou_type_associations
            .get(&type_name.to_lowercase())
            .copied()
            .or_else(|| {
                self.parent_index
                    .and_then(|it| it.find_associated_pou_type_and_debug(type_name))
            })
    }

    pub fn find_associated_pou_type(&self, type_name: &str) -> Option<BasicTypeEnum<'ink>> {
        self.find_associated_pou_type_and_debug(type_name).map(|(it, _) | it)
    }

    pub fn find_associated_pou_debug_type(&self, type_name: &str) -> Option<DIType<'ink>> {
        self.find_associated_pou_type_and_debug(type_name).map(|(_, it) | it).flatten()
    }

    pub fn get_associated_type(&self, type_name: &str) -> Result<BasicTypeEnum<'ink>, Diagnostic> {
        self.find_associated_type(type_name)
            .ok_or_else(|| Diagnostic::unknown_type(type_name, SourceRange::undefined()))
    }

    pub fn get_associated_pou_type(
        &self,
        type_name: &str,
    ) -> Result<BasicTypeEnum<'ink>, Diagnostic> {
        self.find_associated_pou_type(type_name)
            .ok_or_else(|| Diagnostic::unknown_type(type_name, SourceRange::undefined()))
    }

    pub fn find_associated_initial_value(&self, type_name: &str) -> Option<BasicValueEnum<'ink>> {
        self.initial_value_associations
            .get(&type_name.to_lowercase())
            .copied()
            .or_else(|| {
                self.parent_index
                    .and_then(|it| it.find_associated_initial_value(type_name))
            })
    }

    pub fn associate_global(
        &mut self,
        variable_name: &str,
        global_variable: GlobalValue<'ink>,
    ) -> Result<(), Diagnostic> {
        self.global_values
            .insert(variable_name.to_lowercase(), global_variable);
        //TODO  : Remove this and replace it with a lookup into globals where needed
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
    ) -> Result<(), Diagnostic> {
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
                    .and_then(|it| it.find_associated_implementation(callable_name))
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
                    .and_then(|it| it.find_associated_variable_value(qualified_name))
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
                    .and_then(|it| it.find_loaded_associated_variable_value(qualified_name))
            });

        //If nothing got associated, see if we have a global we could reuse
        result.or_else(|| {
            self.find_associated_variable_value(qualified_name)
                .filter(|it| it.is_pointer_value())
                .map(BasicValueEnum::into_pointer_value)
        })
    }

    pub fn find_constant_value(&self, qualified_name: &str) -> Option<BasicValueEnum<'ink>> {
        self.constants.get(qualified_name).copied()
    }

    pub fn associate_utf08_literal(
        &mut self,
        literal: String,
        literal_variable: GlobalValue<'ink>,
    ) {
        self.utf08_literals.insert(literal, literal_variable);
    }

    pub fn find_utf08_literal_string(&self, literal: &str) -> Option<&GlobalValue<'ink>> {
        self.utf08_literals.get(literal).or_else(|| {
            self.parent_index
                .and_then(|it| it.find_utf08_literal_string(literal))
        })
    }

    pub fn associate_utf16_literal(
        &mut self,
        literal: String,
        literal_variable: GlobalValue<'ink>,
    ) {
        self.utf16_literals.insert(literal, literal_variable);
    }

    pub fn find_utf16_literal_string(&self, literal: &str) -> Option<&GlobalValue<'ink>> {
        self.utf16_literals.get(literal).or_else(|| {
            self.parent_index
                .and_then(|it| it.find_utf16_literal_string(literal))
        })
    }
}
