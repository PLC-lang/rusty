use std::collections::HashMap;

use inkwell::types::{BasicTypeEnum};
use inkwell::values::{BasicValueEnum};

#[cfg(test)]
mod tests;
mod visitor;

/// a base index entry
pub struct IndexEntry<T, K> {
    name                    : String,
    information             : T,
    generated_reference     : Option<K>,

}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VariableType { Local, Input, Output, InOut, Global }

/// information regarding a variable
#[derive(Debug)]
pub struct VariableInformation {
    /// the type of variable
    variable_type   : VariableType, 
    /// the variable's datatype
    data_type_name  : String,
}

#[derive(Debug)]
pub enum DataTypeType { 
    Scalar,      // built in types: INT, BOOL, WORD, ... 
    Struct,         // Struct-DataType
    FunctionBlock,  // a Functionblock instance
    AliasType       // a Custom-Alias-dataType 
}

/// information regarding a custom datatype
#[derive(Debug)]
pub struct DataTypeInformation {
    /// what kind of datatype is this
    kind        : DataTypeType,
}

#[derive(Debug,PartialEq)]
pub enum PouKind {
    Program,
    Function,
    FunctionBlock,
}

#[derive(Debug)]
pub struct PouInformation {
    pou_kind: PouKind,
}

pub type VariableIndexEntry<'ctx> = IndexEntry<VariableInformation, BasicValueEnum<'ctx>>;
pub type TypeIndexEntry<'ctx>     = IndexEntry<DataTypeInformation, BasicTypeEnum<'ctx>>;
pub type PouIndexEntry<'ctx>      = IndexEntry<PouInformation, BasicTypeEnum<'ctx>>;

/// The global index of the rusty-compiler
/// 
/// The index contains information about all referencable elements. Furthermore it
/// contains information about the type-system of the compiled program.
/// 
pub struct Index<'ctx> {
    /// all global variables
    global_variables    : HashMap<String, VariableIndexEntry<'ctx>>,

    /// all local variables, grouped by the POU's name
    local_variables     : HashMap<String, HashMap<String, VariableIndexEntry<'ctx>>>,

    /// all POUs
    pous                : HashMap<String, PouIndexEntry<'ctx>>,

    /// all types (structs, enums, type, etc.)
    types               : HashMap<String, TypeIndexEntry<'ctx>>,
}

impl<'ctx> Index<'ctx> {
    pub fn new() -> Index<'ctx> {
        let mut index = Index {
            global_variables : HashMap::new(),
            local_variables : HashMap::new(),
            pous : HashMap::new(),
            types : HashMap::new(),   
        };

        index.types.insert("INT".to_string(), TypeIndexEntry{
            name: "INT".to_string(),
            information: DataTypeInformation {
                kind: DataTypeType::Scalar,
            },
            generated_reference: None,
        });
        index.types.insert("BOOL".to_string(), TypeIndexEntry{
            name: "BOOL".to_string(),
            information: DataTypeInformation {
                kind: DataTypeType::Scalar,
            },
            generated_reference: None,
        });
        index
    }

    pub fn find_global_variable(&self, name: &str) -> Option<&VariableIndexEntry> {
        self.global_variables.get(name)
    }

    pub fn find_pou(&self, name: &str) -> Option<&PouIndexEntry> {
        self.pous.get(name)
    }

    pub fn find_member(&self, pou_name: &str, variable_name: &str) -> Option<&VariableIndexEntry>{
        self.local_variables.get(pou_name)
            .and_then(|map| map.get(variable_name))
    }

    pub fn register_local_variable(&mut self, 
                                        pou_name: String, 
                                        variable_name: String, 
                                        variable_type: VariableType, 
                                        type_name: String) {
        
        let locals = self.local_variables.entry(pou_name).or_insert_with(|| HashMap::new());

        let entry = VariableIndexEntry{
            name : variable_name.clone(),
            information : VariableInformation {
                variable_type: variable_type,
                data_type_name: type_name
            },
            generated_reference: None,
        };                         
        locals.insert(variable_name, entry);



    }

    pub fn register_global_variable(&mut self,
                                name: String, 
                                variable_type : VariableType, 
                                type_name: String){
        
        let entry = VariableIndexEntry{
            name : name.clone(),
            information : VariableInformation {
                variable_type: VariableType::Global,
                data_type_name: type_name
            },
            generated_reference: None,
        };                         
        self.global_variables.insert(name, entry);

    }

    pub fn register_pou(&mut self,
                        pou_name: String,
                        pou_kind: PouKind) {

        let index_entry = PouIndexEntry{
            name: pou_name.clone(),
            generated_reference: None,
            information: PouInformation {
                pou_kind: pou_kind,
            }
        };
        self.pous.insert(pou_name, index_entry);
    }


}