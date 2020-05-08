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

#[derive(Copy, Clone)]
pub enum VariableType { Local, Input, Output, InOut, Global }

/// information regarding a variable
pub struct VariableInformation<'ctx> {
    /// the type of variable
    variable_type   : VariableType, 
    /// the variable's datatype
    data_type       : &'ctx TypeIndexEntry<'ctx>,
    /// true if this variable is a constat
    is_const         : bool,
    /// the number of array-elements, 0 if this is no array
    array_lengths   : u32,  
    /// the offset of the array-index (e.g. 5 if the array was declared [5..10])
    array_offset    : i32,
}

pub enum DataTypeType { 
    Scalar,      // built in types: INT, BOOL, WORD, ... 
    Struct,         // Struct-DataType
    FunctionBlock,  // a Functionblock instance
    AliasType       // a Custom-Alias-dataType 
}

/// information regarding a custom datatype
pub struct DataTypeInformation {
    /// what kind of datatype is this
    kind        : DataTypeType,
}

pub enum PouKind {
    Program,
    Function,
    FunctionBlock,
}

pub struct PouInformation {
    pou_type: PouKind,
}

pub type VariableIndexEntry<'ctx> = IndexEntry<VariableInformation<'ctx>, BasicValueEnum<'ctx>>;
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
    local_variables     : HashMap<String, VariableIndexEntry<'ctx>>,

    /// all POUs
    pous                : HashMap<String, PouIndexEntry<'ctx>>,

    /// all types (structs, enums, type, etc.)
    types               : HashMap<String, TypeIndexEntry<'ctx>>,
}

impl<'ctx> Index<'ctx> {
    pub fn new() -> Index<'ctx> {
        Index {
            global_variables : HashMap::new(),
            local_variables : HashMap::new(),
            pous : HashMap::new(),
            types : HashMap::new(),   
        }
    }

    pub fn register_variable(&mut self,
                                name: String, 
                                variable_type : VariableType, 
                                is_const: bool, 
                                array_lengt : u32, 
                                array_offset : i32, 
                                type_name: String) -> &'static VariableIndexEntry<'static> {

        unimplemented!();
    }

    pub fn register_pou(&mut self,
                        pou_name: String,
                        pou_type: PouKind) {

        let index_entry = PouIndexEntry{
            name: pou_name.clone(),
            generated_reference: None,
            information: PouInformation {
                pou_type: pou_type,
            }
        };
        self.pous.insert(pou_name, index_entry);
    }


}