use global_var_indexer::VarGlobalIndexer;
use implementation_indexer::ImplementationIndexer;
use plc_ast::{
    ast::{CompilationUnit, Implementation, VariableBlockType},
    visitor::{AstVisitor, Walker},
};
use pou_indexer::PouIndexer;
use user_type_indexer::UserTypeIndexer;

use super::Index;

mod global_var_indexer;
mod implementation_indexer;
mod pou_indexer;
mod user_type_indexer;

/// Indexes all symbols found in the given Compiliation Unit
/// and returns the resulting Index
pub fn index(unit: &CompilationUnit) -> Index {
    let mut indexer = SymbolIndexer::default();
    unit.walk(&mut indexer);
    indexer.index
}

/// Indexer that registers all symbols in the index
#[derive(Default)]
pub struct SymbolIndexer {
    pub index: Index,
}

/// The SymbolIndexer is responsible for registering all delcared types and symbols in the index.
impl AstVisitor for SymbolIndexer {
    /// Visits a VAR_GLOBAL VariableBlock and registers all variables as globals in the index
    fn visit_variable_block(&mut self, block: &plc_ast::ast::VariableBlock) {
        if block.variable_block_type == VariableBlockType::Global {
            // let the global var indexer handle the global variables
            let mut indexer = VarGlobalIndexer::new(block.constant, block.linkage, &mut self.index);
            for var in &block.variables {
                indexer.visit_variable(var);
            }
        }
    }

    /// Visits a user type declaration
    /// Registers the user type in the index using the UserTypeIndexer
    fn visit_user_type_declaration(&mut self, user_type: &plc_ast::ast::UserTypeDeclaration) {
        UserTypeIndexer::new(&mut self.index, user_type).visit_user_type_declaration(user_type);
    }

    /// Visits a pou and registers all member variables in the index
    /// Also registers the pou's struct type in the index
    fn visit_pou(&mut self, pou: &plc_ast::ast::Pou) {
        PouIndexer::new(&mut self.index).visit_pou(pou);
    }

    /// Visits an implementation and registers the implementation in the index
    fn visit_implementation(&mut self, implementation: &Implementation) {
        ImplementationIndexer::new(&mut self.index).index_implementation(implementation);
    }

    fn visit_config_variable(&mut self, config_variable: &plc_ast::ast::ConfigVariable) {
        self.index.config_variables.push(config_variable.clone());
    }
}
