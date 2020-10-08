/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use std::collections::HashMap;

use crate::ast::CompilationUnit;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{FunctionValue, PointerValue};

mod pre_processor;
#[cfg(test)]
mod tests;
mod visitor;

#[derive(Debug, Clone)]
pub enum DataTypeInformation<'ctx> {
    Struct {
        name: String,
        generated_type: BasicTypeEnum<'ctx>,
    },
    Integer {
        signed: bool,
        size: u32,
        generated_type: BasicTypeEnum<'ctx>,
    },
    Float {
        size: u32,
        generated_type: BasicTypeEnum<'ctx>,
    },
    String {
        size: u32,
        generated_type: BasicTypeEnum<'ctx>,
    }, 
}

impl<'ctx> DataTypeInformation<'ctx> {
    pub fn is_int(&self) -> bool {
        if let DataTypeInformation::Integer { .. } = self {
            true
        } else {
            false
        }
    }

    pub fn is_float(&self) -> bool {
        if let DataTypeInformation::Float { .. } = self {
            true
        } else {
            false
        }
    }

    pub fn is_numerical(&self) -> bool {
        match self {
            DataTypeInformation::Integer { .. } | DataTypeInformation::Float { .. } => true,
            _ => false,
        }
    }

    pub fn get_type(&self) -> BasicTypeEnum<'ctx> {
        match self {
            DataTypeInformation::Integer { generated_type, .. } => *generated_type,
            DataTypeInformation::Float { generated_type, .. } => *generated_type,
            DataTypeInformation::String { generated_type, .. } => *generated_type,
            DataTypeInformation::Struct { generated_type, .. } => *generated_type,
        }
    }

    pub fn get_size(&self) -> u32 {
        match self {
            DataTypeInformation::Integer { size, .. } => *size,
            DataTypeInformation::Float { size, .. } => *size,
            DataTypeInformation::String { size, .. } => *size,
            DataTypeInformation::Struct { .. } => 0,
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct VariableIndexEntry<'ctx> {
    name: String,
    information: VariableInformation,
    generated_reference: Option<PointerValue<'ctx>>,
}

#[derive(Debug)]
pub struct DataTypeIndexEntry<'ctx> {
    name: String,
    implementation: Option<FunctionValue<'ctx>>, // the generated function to call if this type is callable
    information: Option<DataTypeInformation<'ctx>>,
}

impl<'ctx> VariableIndexEntry<'ctx> {
    pub fn associate(&mut self, generated_reference: PointerValue<'ctx>) {
        self.generated_reference = Some(generated_reference);
    }

    pub fn get_type_name(&self) -> &str {
        self.information.data_type_name.as_str()
    }

    pub fn get_generated_reference(&self) -> Option<PointerValue<'ctx>> {
        self.generated_reference
    }

    pub fn get_location_in_parent(&self) -> Option<u32> {
        self.information.location
    }
}
impl<'ctx> DataTypeIndexEntry<'ctx> {
    pub fn associate_implementation(&mut self, implementation: FunctionValue<'ctx>) {
        self.implementation = Some(implementation);
    }

    pub fn get_type(&self) -> Option<BasicTypeEnum<'ctx>> {
        self.information.as_ref().map(|it| it.get_type())
    }
    
    pub fn get_implementation(&self) -> Option<FunctionValue<'ctx>> {
        self.implementation
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_type_information(&self) -> Option<&DataTypeInformation<'ctx>> {
        self.information.as_ref()
    }

    pub fn clone_type_information(&self) -> Option<DataTypeInformation<'ctx>> {
        self.information.clone()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VariableType {
    Local,
    Input,
    Output,
    InOut,
    Global,
    Return,
}

/// information regarding a variable
#[derive(Debug, PartialEq)]
pub struct VariableInformation {
    /// the type of variable
    variable_type: VariableType,
    /// the variable's datatype
    data_type_name: String,
    /// the variable's qualifier
    qualifier: Option<String>,
    /// Location in the qualifier
    location: Option<u32>,
}

#[derive(Debug)]
pub enum DataTypeType {
    Scalar,        // built in types: INT, BOOL, WORD, ...
    Struct,        // Struct-DataType
    FunctionBlock, // a Functionblock instance
    AliasType,     // a Custom-Alias-dataType
}

/// The global index of the rusty-compiler
///
/// The index contains information about all referencable elements. Furthermore it
/// contains information about the type-system of the compiled program.
///
/// TODO: consider String-references
///
#[derive(Debug)]
pub struct Index<'ctx> {
    /// all global variables
    global_variables: HashMap<String, VariableIndexEntry<'ctx>>,

    /// all local variables, grouped by the POU's name
    local_variables: HashMap<String, HashMap<String, VariableIndexEntry<'ctx>>>,

    /// all types (structs, enums, type, POUs, etc.)
    types: HashMap<String, DataTypeIndexEntry<'ctx>>,

    void_type: DataTypeIndexEntry<'ctx>,
}

impl<'ctx> Index<'ctx> {
    pub fn new() -> Index<'ctx> {
        let index = Index {
            global_variables: HashMap::new(),
            local_variables: HashMap::new(),
            types: HashMap::new(),
            void_type: DataTypeIndexEntry {
                name: "void".to_string(),
                implementation: None,
                information: None,
            },
        };
        index
    }

    pub fn get_void_type(&self) -> &DataTypeIndexEntry<'ctx> {
        &self.void_type
    }

    pub fn find_global_variable(&self, name: &str) -> Option<&VariableIndexEntry<'ctx>> {
        self.global_variables.get(name)
    }

    pub fn find_member(
        &self,
        pou_name: &str,
        variable_name: &str,
    ) -> Option<&VariableIndexEntry<'ctx>> {
        self.local_variables
            .get(pou_name)
            .and_then(|map| map.get(variable_name))
    }

    pub fn find_input_parameter(&self, pou_name : &str, index : u32) -> Option<&VariableIndexEntry<'ctx>> {
        self.local_variables.get(pou_name)
            .and_then(|map| map.values().filter(|item| item.information.variable_type == VariableType::Input).find(|item| item.information.location.unwrap() == index))
    }

    //                                     none                 ["myGlobal", "a", "b"]
    pub fn find_variable(
        &self,
        context: Option<&str>,
        segments: &[String],
    ) -> Option<&VariableIndexEntry<'ctx>> {
        if segments.is_empty() {
            return None;
        }

        let first_var = &segments[0];

        let mut result = match context {
            Some(context) => self
                .find_member(context, first_var)
                .or_else(|| self.find_global_variable(first_var)),
            None => self.find_global_variable(first_var),
        };
        for segment in segments.iter().skip(1) {
            result = match result {
                Some(context) => self.find_member(&context.information.data_type_name, &segment),
                None => None,
            };
        }
        result
    }

    pub fn find_type(&self, type_name: &str) -> Option<&DataTypeIndexEntry<'ctx>> {
        self.types.get(type_name)
    }

    pub fn find_type_information(&self, type_name: &str) -> Option<DataTypeInformation<'ctx>> {
        self.find_type(type_name)
            .and_then(|entry| entry.clone_type_information())
    }

    pub fn find_callable_instance_variable(
        &self,
        context: Option<&str>,
        reference: &[String],
    ) -> Option<&VariableIndexEntry<'ctx>> {
        //look for a *callable* variable with that name
        self.find_variable(context, reference).filter(|v| {
            //callable means, there is an implementation associated with the variable's datatype
            self.find_type(v.information.data_type_name.as_str())
                .map(|it| it.implementation)
                .flatten()
                .is_some()
        })
    }

    pub fn register_local_variable(
        &mut self,
        pou_name: String,
        variable_name: String,
        variable_type: VariableType,
        type_name: String,
        location: u32,
    ) {
        let locals = self
            .local_variables
            .entry(pou_name.clone())
            .or_insert_with(|| HashMap::new());

        let entry = VariableIndexEntry {
            name: variable_name.clone(),
            information: VariableInformation {
                variable_type: variable_type,
                data_type_name: type_name,
                qualifier: Some(pou_name.clone()),
                location: Some(location),
            },
            generated_reference: None,
        };
        locals.insert(variable_name, entry);
    }

    pub fn associate_local_variable(
        &mut self,
        pou_name: &str,
        variable_name: &str,
        value: PointerValue<'ctx>,
    ) {
        if let Some(entry) = self
            .local_variables
            .get_mut(pou_name)
            .and_then(|map| map.get_mut(variable_name))
        {
            entry.generated_reference = Some(value);
        }
    }

    pub fn register_global_variable(&mut self, name: String, type_name: String) {
        let entry = VariableIndexEntry {
            name: name.clone(),
            information: VariableInformation {
                variable_type: VariableType::Global,
                data_type_name: type_name,
                qualifier: None,
                location: None,
            },
            generated_reference: None,
        };
        self.global_variables.insert(name, entry);
    }

    pub fn associate_global_variable(&mut self, name: &str, value: PointerValue<'ctx>) {
        if let Some(entry) = self.global_variables.get_mut(name) {
            entry.generated_reference = Some(value);
        }
    }

    pub fn associate_callable_implementation(&mut self, name: &str, value: FunctionValue<'ctx>) {
        if let Some(entry) = self.types.get_mut(name) {
            entry.implementation = Some(value);
        };
    }

    pub fn associate_type(&mut self, name: &str, data_type_information: DataTypeInformation<'ctx>) {
        if let Some(entry) = self.types.get_mut(name) {
            entry.information = Some(data_type_information);
        };
    }

    pub fn print_global_variables(&self) {
        println!("{:?}", self.global_variables);
    }

    pub fn register_type(&mut self, type_name: String) {
        let index_entry = DataTypeIndexEntry {
            name: type_name.clone(),
            implementation: None,
            information: None,
        };
        self.types.insert(type_name, index_entry);
    }

    pub fn visit(&mut self, unit: &mut CompilationUnit) {
        visitor::visit(self, unit);
    }

    //TODO does this belong into the index?
    pub fn pre_process(&mut self, unit: &mut CompilationUnit) {
        pre_processor::pre_process(unit);
    }
}
