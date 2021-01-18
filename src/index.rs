/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use std::collections::HashMap;

use crate::ast::CompilationUnit;
use inkwell::{types::BasicTypeEnum, values::BasicValueEnum};
use inkwell::values::{FunctionValue, PointerValue};

mod pre_processor;
mod visitor;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct Dimension {
    pub start_offset: i32,
    pub end_offset : i32,
}

impl Dimension {
    pub fn get_length(&self) -> u32 {
        (self.end_offset - self.start_offset + 1) as u32
    }
}

#[derive(Debug, Clone)]
pub enum DataTypeInformation<'ctx> {
    Struct {
        name: String,
        generated_type: BasicTypeEnum<'ctx>,
    },
    Array {
        name: String,
        internal_type_information : Box<DataTypeInformation<'ctx>>,
        generated_type: BasicTypeEnum<'ctx>,
        dimensions : Vec<Dimension>, 
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
    Alias {
        name: String,
        referenced_type: String,
    }
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
            DataTypeInformation::Array { generated_type, .. } => *generated_type,
            DataTypeInformation::Alias { .. } => unimplemented!(),
        }
    }

    pub fn get_size(&self) -> u32 {
        match self {
            DataTypeInformation::Integer { size, .. } => *size,
            DataTypeInformation::Float { size, .. } => *size,
            DataTypeInformation::String { size, .. } => *size,
            DataTypeInformation::Struct { .. } => 0, //TODO : Should we fill in the struct members here for size calculation or save the struct size.
            DataTypeInformation::Array { .. } => unimplemented!(), //Propably length * inner type size
            DataTypeInformation::Alias { .. } => unimplemented!(),
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
    /// the initial value defined on the TYPE-declration
    initial_value: Option<BasicValueEnum<'ctx>>,
    information: Option<DataTypeInformation<'ctx>>,
}

impl<'ctx> VariableIndexEntry<'ctx> {

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

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
    pub fn get_type(&self) -> Option<BasicTypeEnum<'ctx>> {
        self.information.as_ref().map(|it| it.get_type())
    }

    pub fn get_implementation(&self) -> Option<FunctionValue<'ctx>> {
        self.implementation
    }

    pub fn get_initial_value(&self) -> Option<BasicValueEnum<'ctx>> {
        self.initial_value
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
    member_variables: HashMap<String, HashMap<String, VariableIndexEntry<'ctx>>>,

    /// all types (structs, enums, type, POUs, etc.)
    types: HashMap<String, DataTypeIndexEntry<'ctx>>,

    void_type: DataTypeIndexEntry<'ctx>,
}

impl<'ctx> Index<'ctx> {
    pub fn new() -> Index<'ctx> {
        let index = Index {
            global_variables: HashMap::new(),
            member_variables: HashMap::new(),
            types: HashMap::new(),
            void_type: DataTypeIndexEntry {
                name: "void".to_string(),
                implementation: None,
                information: None,
                initial_value: None
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
        self.member_variables
            .get(pou_name)
            .and_then(|map| map.get(variable_name))
    }

    pub fn find_input_parameter(&self, pou_name : &str, index : u32) -> Option<&VariableIndexEntry<'ctx>> {
        self.member_variables.get(pou_name)
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
        let data_type = self.types.get(type_name);
        data_type.map(|it| {
            if let Some(DataTypeInformation::Alias { referenced_type, .. }) = &it.information {
                return self.find_type(referenced_type.as_str());
            }
            Some(it)
        }).flatten()
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

    /// registers a member-variable of a container to be accessed in a qualified name.
    /// e.g. "POU.member", "StructName.member", etc.
    ///
    /// #Arguments
    /// * `container_name`- the name of hosting container (pou or struct)
    /// * `variable_name` - the name of the member variable
    /// * `variable_linkage` - the linkage-type of that variable (one of local, global, etc. )
    /// * `variable_type_name` - the variable's data type as a string
    /// * `location` - the location (index) inside the container
    pub fn register_member_variable(
        &mut self,
        container_name: String,
        variable_name: String,
        variable_linkage: VariableType,
        variable_type_name: String,
        location: u32,
    ) {
        let members = self
            .member_variables
            .entry(container_name.clone())
            .or_insert_with(|| HashMap::new());

        let entry = VariableIndexEntry {
            name: variable_name.clone(),
            information: VariableInformation {
                variable_type: variable_linkage,
                data_type_name: variable_type_name,
                qualifier: Some(container_name.clone()),
                location: Some(location),
            },
            generated_reference: None,
        };
        members.insert(variable_name, entry);
    }

    pub fn associate_local_variable(
        &mut self,
        pou_name: &str,
        variable_name: &str,
        value: PointerValue<'ctx>,
    ) {
        if let Some(entry) = self
            .member_variables
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

    pub fn associate_type_alias(&mut self, alias_name: &str, referenced_type: &str) {
        if let Some(entry) = self.find_type_information(referenced_type) {
            self.associate_type(alias_name, entry);
        }
    }

    pub fn associate_type_initial_value(&mut self, name: &str, initial_value: BasicValueEnum<'ctx>) {
        if let Some(entry) = self.types.get_mut(name) {
            entry.initial_value = Some(initial_value);
        }
    }

    pub fn print_global_variables(&self) {
        println!("{:?}", self.global_variables);
    }

    pub fn register_type(&mut self, type_name: String) {
        let index_entry = DataTypeIndexEntry {
            name: type_name.clone(),
            implementation: None,
            information: None,
            initial_value: None,
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
