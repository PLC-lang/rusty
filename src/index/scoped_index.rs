use std::{collections::HashMap, rc::Rc};

use plc_ast::provider::IdProvider;
use plc_source::source_location::SourceLocation;

use crate::typesystem::DataType;

use super::VariableIndexEntry;

/// A minimal index implementation that can be used for local scopes
#[derive(Debug, Clone)]
pub struct ScopedIndex {
    ///The scope of the current index, this is usually a POU
    scope: String,

    /// A unique identifier that new variables in this scope will inherit
    suffix_provider: IdProvider,

    /// The location that caused this scope to be created
    start_location: SourceLocation,

    /// New variables defined by this index
    variables: HashMap<String, VariableIndexEntry>,

    /// Datatypes defined by this index
    type_index: HashMap<String, DataType>,

    parent: Option<Rc<ScopedIndex>>,
}

impl ScopedIndex {
    pub fn merge_into(self, target: &mut Self) {
        target.variables.extend(self.variables);
        target.type_index.extend(self.type_index);
    }

    pub fn add_variable(&mut self, _name: &str) {}

    pub fn add_type(&mut self, _name: &str) {}

    pub fn find_variable(&self, _name: &str) -> Option<&VariableIndexEntry> {
        todo!()
    }

    pub fn find_type(&self, _name: &str) -> Option<&DataType> {
        todo!()
    }

    pub fn new(
        parent: Option<Rc<ScopedIndex>>,
        location: SourceLocation,
        scope: &str,
        suffix_provider: IdProvider,
    ) -> ScopedIndex {
        ScopedIndex {
            scope: scope.to_string(),
            suffix_provider,
            start_location: location,
            parent,
            type_index: Default::default(),
            variables: Default::default(),
        }
    }
}
