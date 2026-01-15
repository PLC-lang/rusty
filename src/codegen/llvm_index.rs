// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use inkwell::types::{AnyTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{AnyValue, BasicValueEnum, FunctionValue, GlobalValue, PointerValue};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;
use plc_util::convention::qualified_name;
use rustc_hash::FxHashMap;

use super::CodegenError;

/// Index view containing declared values for the current context
/// Parent Index is the a fallback lookup index for values not declared locally
#[derive(Debug, Clone, Default)]
pub struct LlvmTypedIndex<'ink> {
    parent_index: Option<&'ink LlvmTypedIndex<'ink>>,
    type_associations: FxHashMap<String, AnyTypeEnum<'ink>>,
    pou_type_associations: FxHashMap<String, AnyTypeEnum<'ink>>,
    global_values: FxHashMap<String, GlobalValue<'ink>>,
    got_indices: FxHashMap<String, u64>,
    initial_value_associations: FxHashMap<String, BasicValueEnum<'ink>>,
    loaded_variable_associations: FxHashMap<String, PointerValue<'ink>>,
    implementations: FxHashMap<String, FunctionValue<'ink>>,
    utf08_literals: FxHashMap<String, GlobalValue<'ink>>,
    utf16_literals: FxHashMap<String, GlobalValue<'ink>>,
}

pub trait TypeHelper<'ink> {
    #[allow(clippy::wrong_self_convention)]
    fn as_basic_type(self) -> Option<BasicTypeEnum<'ink>>;
}

impl<'ink> TypeHelper<'ink> for AnyTypeEnum<'ink> {
    fn as_basic_type(self) -> Option<BasicTypeEnum<'ink>> {
        match self {
            AnyTypeEnum::ArrayType(value) => Some(value.as_basic_type_enum()),
            AnyTypeEnum::FloatType(value) => Some(value.as_basic_type_enum()),
            AnyTypeEnum::IntType(value) => Some(value.as_basic_type_enum()),
            AnyTypeEnum::PointerType(value) => Some(value.as_basic_type_enum()),
            AnyTypeEnum::StructType(value) => Some(value.as_basic_type_enum()),
            AnyTypeEnum::VectorType(value) => Some(value.as_basic_type_enum()),
            AnyTypeEnum::ScalableVectorType(value) => Some(value.as_basic_type_enum()),
            AnyTypeEnum::VoidType(_) => None,
            AnyTypeEnum::FunctionType(_) => None,
        }
    }
}

impl<'ink> LlvmTypedIndex<'ink> {
    pub fn create_child(parent: &'ink LlvmTypedIndex<'ink>) -> LlvmTypedIndex<'ink> {
        LlvmTypedIndex {
            parent_index: Some(parent),
            type_associations: FxHashMap::default(),
            pou_type_associations: FxHashMap::default(),
            global_values: FxHashMap::default(),
            got_indices: FxHashMap::default(),
            initial_value_associations: FxHashMap::default(),
            loaded_variable_associations: FxHashMap::default(),
            implementations: FxHashMap::default(),
            utf08_literals: FxHashMap::default(),
            utf16_literals: FxHashMap::default(),
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
        for (name, index) in other.got_indices.drain() {
            self.got_indices.insert(name, index);
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
        self.utf08_literals.extend(other.utf08_literals);
        self.utf16_literals.extend(other.utf16_literals);
    }

    pub fn associate_type(
        &mut self,
        type_name: &str,
        target_type: AnyTypeEnum<'ink>,
    ) -> Result<(), CodegenError> {
        let name = type_name.to_lowercase();

        log::trace!("registered `{name}` as type `{}`", target_type.print_to_string());
        self.type_associations.insert(name, target_type);
        Ok(())
    }

    pub fn associate_pou_type(
        &mut self,
        type_name: &str,
        target_type: AnyTypeEnum<'ink>,
    ) -> Result<(), CodegenError> {
        let name = type_name.to_lowercase();

        log::trace!("registered `{name}` as POU type `{}`", target_type.print_to_string());
        self.pou_type_associations.insert(name, target_type);
        Ok(())
    }

    pub fn associate_initial_value(
        &mut self,
        type_name: &str,
        initial_value: BasicValueEnum<'ink>,
    ) -> Result<(), CodegenError> {
        let name = type_name.to_lowercase();

        log::trace!("registered `{name}` as initial value type `{}`", initial_value.print_to_string());
        self.initial_value_associations.insert(name, initial_value);
        Ok(())
    }

    pub fn associate_loaded_local_variable(
        &mut self,
        container_name: &str,
        variable_name: &str,
        target_value: PointerValue<'ink>,
    ) -> Result<(), CodegenError> {
        let name = qualified_name(container_name, variable_name).to_lowercase();

        log::trace!("registered `{name}` as loaded local type `{}`", target_value.print_to_string());
        self.loaded_variable_associations.insert(name, target_value);
        Ok(())
    }

    pub fn associate_global(
        &mut self,
        variable_name: &str,
        global_variable: GlobalValue<'ink>,
    ) -> Result<(), CodegenError> {
        let name = variable_name.to_lowercase();

        log::trace!("registered `{name}` as global variable type `{}`", global_variable.print_to_string());
        self.global_values.insert(name.clone(), global_variable);
        self.initial_value_associations.insert(name, global_variable.as_pointer_value().into());

        // FIXME: Do we want to call .insert_new_got_index() here?
        Ok(())
    }

    pub fn associate_implementation(
        &mut self,
        callable_name: &str,
        function_value: FunctionValue<'ink>,
    ) -> Result<(), CodegenError> {
        let name = callable_name.to_lowercase();

        log::trace!("registered `{name}` as implementation type `{}`", function_value.print_to_string());
        self.implementations.insert(name, function_value);

        Ok(())
    }

    pub fn associate_utf08_literal(&mut self, literal: &str, literal_variable: GlobalValue<'ink>) {
        log::trace!("registered literal {literal}");
        self.utf08_literals.insert(literal.to_string(), literal_variable);
    }

    pub fn associate_utf16_literal(&mut self, literal: &str, literal_variable: GlobalValue<'ink>) {
        log::trace!("registered literal {literal}");
        self.utf16_literals.insert(literal.to_string(), literal_variable);
    }

    pub fn associate_got_index(&mut self, variable_name: &str, index: u64) -> Result<(), CodegenError> {
        let name = variable_name.to_lowercase();
        self.got_indices.insert(name, index);

        Ok(())
    }

    pub fn find_global_value(&self, name: &str) -> Option<GlobalValue<'ink>> {
        self.global_values
            .get(&name.to_lowercase())
            .copied()
            .or_else(|| self.parent_index.and_then(|it| it.find_global_value(name)))
    }

    pub fn find_got_index(&self, name: &str) -> Option<u64> {
        self.got_indices
            .get(&name.to_lowercase())
            .copied()
            .or_else(|| self.parent_index.and_then(|it| it.find_got_index(name)))
    }

    pub fn find_associated_type(&self, type_name: &str) -> Option<BasicTypeEnum<'ink>> {
        self.type_associations
            .get(&type_name.to_lowercase())
            .copied()
            .and_then(TypeHelper::as_basic_type)
            .or_else(|| self.parent_index.and_then(|it| it.find_associated_type(type_name)))
            .or_else(|| self.find_associated_pou_type(type_name))
    }

    pub fn find_associated_pou_type(&self, type_name: &str) -> Option<BasicTypeEnum<'ink>> {
        self.pou_type_associations
            .get(&type_name.to_lowercase())
            .copied()
            .and_then(TypeHelper::as_basic_type)
            .or_else(|| self.parent_index.and_then(|it| it.find_associated_pou_type(type_name)))
    }

    pub fn get_associated_type(&self, type_name: &str) -> Result<BasicTypeEnum<'ink>, CodegenError> {
        self.find_associated_type(type_name)
            .ok_or_else(|| Diagnostic::unknown_type(type_name, SourceLocation::undefined()).into())
    }

    pub fn get_associated_pou_type(&self, type_name: &str) -> Result<BasicTypeEnum<'ink>, CodegenError> {
        self.find_associated_pou_type(type_name)
            .ok_or_else(|| Diagnostic::unknown_type(type_name, SourceLocation::undefined()).into())
    }

    pub fn find_associated_initial_value(&self, type_name: &str) -> Option<BasicValueEnum<'ink>> {
        self.initial_value_associations
            .get(&type_name.to_lowercase())
            .copied()
            .or_else(|| self.parent_index.and_then(|it| it.find_associated_initial_value(type_name)))
    }

    pub fn insert_new_got_index(&mut self, variable_name: &str) -> Result<(), CodegenError> {
        let idx = self.got_indices.values().max().copied().unwrap_or(0);

        self.got_indices.insert(variable_name.to_lowercase(), idx);

        Ok(())
    }

    pub fn find_associated_implementation(&self, callable_name: &str) -> Option<FunctionValue<'ink>> {
        self.implementations
            .get(&callable_name.to_lowercase())
            .copied()
            .or_else(|| self.parent_index.and_then(|it| it.find_associated_implementation(callable_name)))
    }

    pub fn find_associated_variable_value(&self, qualified_name: &str) -> Option<BasicValueEnum<'ink>> {
        self.initial_value_associations
            .get(&qualified_name.to_lowercase())
            .copied()
            .or_else(|| self.parent_index.and_then(|it| it.find_associated_variable_value(qualified_name)))
    }

    pub fn find_loaded_associated_variable_value(&self, qualified_name: &str) -> Option<PointerValue<'ink>> {
        let result =
            self.loaded_variable_associations.get(&qualified_name.to_lowercase()).copied().or_else(|| {
                self.parent_index.and_then(|it| it.find_loaded_associated_variable_value(qualified_name))
            });

        //If nothing got associated, see if we have a global we could reuse
        result.or_else(|| {
            self.find_associated_variable_value(qualified_name)
                .filter(|it| it.is_pointer_value())
                .map(BasicValueEnum::into_pointer_value)
        })
    }

    pub fn find_utf08_literal_string(&self, literal: &str) -> Option<&GlobalValue<'ink>> {
        self.utf08_literals
            .get(literal)
            .or_else(|| self.parent_index.and_then(|it| it.find_utf08_literal_string(literal)))
    }

    pub fn find_utf16_literal_string(&self, literal: &str) -> Option<&GlobalValue<'ink>> {
        self.utf16_literals
            .get(literal)
            .or_else(|| self.parent_index.and_then(|it| it.find_utf16_literal_string(literal)))
    }
}
