/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use indexmap::IndexMap;

use crate::{ast::Statement, compile_error::CompileError, typesystem::*};

pub mod visitor;
#[cfg(test)]
mod tests;



#[derive(Debug, PartialEq)]
pub struct VariableIndexEntry {
    name: String,
    pub initial_value: Option<Statement>,
    information: VariableInformation,
}


impl VariableIndexEntry {

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_qualified_name(&self) -> String {
        if let Some(container_name) = &self.information.qualifier {
            format!("{}.{}", container_name, self.name)
        } else {
            self.name.clone()
        }
    }

    pub fn get_type_name(&self) -> &str {
        self.information.data_type_name.as_str()
    }

    pub fn get_location_in_parent(&self) -> u32 {
        self.information.location
    }

    pub fn is_return(&self) -> bool {
        self.information.variable_type == VariableType::Return
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
    /// the variable's qualifier, None for global variables
    qualifier: Option<String>,
    /// Location in the qualifier defautls to 0 (Single variables)
    location: u32,
}

#[derive(Debug)]
pub enum DataTypeType {
    Scalar,        // built in types: INT, BOOL, WORD, ...
    Struct,        // Struct-DataType
    FunctionBlock, // a Functionblock instance
    AliasType,     // a Custom-Alias-dataType
}

#[derive(Debug)]
pub struct ImplementationIndexEntry {
    call_name : String,
    type_name : String,
}

impl ImplementationIndexEntry {
    pub fn get_call_name(&self) -> &str {
        &self.call_name
    }
    pub fn get_type_name(&self) -> &str {
        &self.type_name
    }
}


/// The global index of the rusty-compiler
///
/// The index contains information about all referencable elements. 
///
/// TODO: consider String-references
///
#[derive(Debug)]
pub struct GlobalIndex {
    /// all global variables
    global_variables: IndexMap<String, VariableIndexEntry>,

    /// all local variables, grouped by the POU's name
    member_variables: IndexMap<String, IndexMap<String, VariableIndexEntry>>,
    
    /// all types (structs, enums, type, POUs, etc.)
    types: IndexMap<String, DataType>,

    /// all implementations
    implementations : IndexMap<String, ImplementationIndexEntry>,

    void_type: DataType,
}

impl GlobalIndex {
    pub fn new() -> GlobalIndex {
        let index = GlobalIndex {
            global_variables: IndexMap::new(),
            member_variables: IndexMap::new(),
            types: IndexMap::new(),
            implementations: IndexMap::new(),
            void_type: DataType {
                name: "void".to_string(),
                initial_value : None,
                information: DataTypeInformation::Void,
            },
        };
        index
    }

    pub fn get_void_type(&self) -> &DataType {
        &self.void_type
    }

    pub fn find_global_variable(&self, name: &str) -> Option<&VariableIndexEntry> {
        self.global_variables.get(name)
    }

    pub fn find_member(
        &self,
        pou_name: &str,
        variable_name: &str,
    ) -> Option<&VariableIndexEntry> {
        self.member_variables
            .get(pou_name)
            .and_then(|map| map.get(variable_name))
    }

    pub fn find_input_parameter(&self, pou_name : &str, index : u32) -> Option<&VariableIndexEntry> {
        self.member_variables.get(pou_name)
            .and_then(|map| map.values().filter(|item| item.information.variable_type == VariableType::Input).find(|item| item.information.location == index))
    }

    //                                     none                 ["myGlobal", "a", "b"]
    pub fn find_variable(
        &self,
        context: Option<&str>,
        segments: &[String],
    ) -> Option<&VariableIndexEntry> {
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

    pub fn find_type(&self, type_name: &str) -> Option<&DataType> {
        let data_type = self.types.get(type_name);
        data_type.map(|it| {
            if let DataTypeInformation::Alias { referenced_type, .. } = it.get_type_information() {
                return self.find_type(referenced_type.as_str());
            }
            Some(it)
        }).flatten()
    }

    
    pub fn get_type(&self, type_name: &str) -> Result<&DataType, CompileError> {
        self.find_type(type_name).ok_or_else(|| CompileError::unknown_type(type_name, 0..0))
    }

    pub fn find_return_variable(&self, pou_name : &str) -> Option<&VariableIndexEntry> {
        let members = self.member_variables.get(pou_name);//.ok_or_else(||CompileError::unknown_type(pou_name, 0..0))?;
        if let Some(members) = members {
            for (_, variable) in members {
                if variable.information.variable_type == VariableType::Return {
                    return Some(variable)
                }
            }
        }
        None
    }

    pub fn find_return_type(&self, pou_name : &str) -> Option<&DataType> {
        let variable = self.find_return_variable(pou_name);
        variable.map(|it| self.get_type(it.get_type_name()).unwrap())
    }

    pub fn find_type_information(&self, type_name: &str) -> Option<DataTypeInformation> {
        self.find_type(type_name)
            .map(|entry| entry.clone_type_information())
    }

    pub fn get_type_information(&self, type_name: &str) -> Result<DataTypeInformation, CompileError> {
        self.find_type_information(type_name).ok_or_else(|| CompileError::unknown_type(type_name, 0..0))
    }
    
    pub fn get_types<'a>(&'a self) -> &'a IndexMap<String, DataType> {
        &self.types
    }

    pub fn get_globals(&self) -> &IndexMap<String, VariableIndexEntry> {
        &self.global_variables
    }

    pub fn get_implementations<'a>(&'a self) -> &'a IndexMap<String, ImplementationIndexEntry> {
        &self.implementations
    }

    pub fn register_implementation(&mut self, call_name : &str, type_name: &str) {
        self.implementations.insert(call_name.into(), ImplementationIndexEntry {
            call_name: call_name.into(),type_name : type_name.into()
        });
    }

    pub fn find_implementation(&self, call_name : &str) -> Option<&ImplementationIndexEntry> {
        self.implementations.get(call_name)
    }

   


    /// registers a member-variable of a container to be accessed in a qualified name.
    /// e.g. "POU.member", "StructName.member", etc.
    ///
    /// #Arguments
    /// * `container_name`- the name of hosting container (pou or struct)
    /// * `variable_name` - the name of the member variable
    /// * `variable_linkage` - the linkage-type of that variable (one of local, global, etc. )
    /// * `variable_type_name` - the variable's data type as a string
    /// * `initial_value` - the initial value as defined in the AST
    /// * `location` - the location (index) inside the container
    pub fn register_member_variable(
        &mut self,
        container_name: &str,
        variable_name: &str,
        variable_linkage: VariableType,
        variable_type_name: &str,
        initial_value : Option<Statement>,
        location: u32,
    ) {
        let members = self
            .member_variables
            .entry(container_name.into())
            .or_insert_with(|| IndexMap::new());

        let entry = VariableIndexEntry {
            name: variable_name.into(),
            initial_value,
            information: VariableInformation {
                variable_type: variable_linkage,
                data_type_name: variable_type_name.into(),
                qualifier: Some(container_name.into()),
                location,
            },
        };
        members.insert(variable_name.into(), entry);
    }

    pub fn register_global_variable(&mut self, name: &str, type_name: &str, initial_value : Option<Statement>) {
        self.register_global_variable_with_name(name, name, type_name, initial_value);
    }

    pub fn register_global_variable_with_name(&mut self, association_name: &str,variable_name : &str, type_name: &str, initial_value : Option<Statement>) {
        let entry = VariableIndexEntry {
            name: variable_name.into(),
            initial_value,
            information: VariableInformation {
                variable_type: VariableType::Global,
                data_type_name: type_name.into(),
                qualifier: None,
                location: 0,
            },
        };
        self.global_variables.insert(association_name.into(), entry);
    }

   
    pub fn print_global_variables(&self) {
        println!("{:?}", self.global_variables);
    }

    pub fn register_type(&mut self, type_name: &str, initial_value : Option<Statement>, information: DataTypeInformation) {
        let index_entry = DataType {
            name : type_name.into(),
            initial_value,
            information,
        };
        self.types.insert(type_name.into(), index_entry);
    }

    pub fn find_callable_instance_variable(
       &self,
       context: Option<&str>,
       reference: &[String],
    ) -> Option<&VariableIndexEntry> {
       //look for a *callable* variable with that name
       self.find_variable(context, reference).filter(|v| {
           //callable means, there is an implementation associated with the variable's datatype
           self.find_implementation(&v.information.data_type_name).is_some()
       })
    }

}

