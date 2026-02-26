use global_var_indexer::VarGlobalIndexer;
use implementation_indexer::ImplementationIndexer;
use plc_ast::{
    ast::{CompilationUnit, Implementation, Interface, PropertyBlock, VariableBlockType},
    visitor::{AstVisitor, Walker},
};
use pou_indexer::PouIndexer;
use user_type_indexer::UserTypeIndexer;

use plc_ast::ast::TypeNature;

use super::{ImplementationType, Index, InterfaceIndexEntry};
use crate::typesystem::{DataType, DataTypeInformation};

mod global_var_indexer;
mod implementation_indexer;
pub mod pou_indexer;
mod user_type_indexer;

/// Indexes all symbols found in the given Compiliation Unit
/// and returns the resulting Index
pub fn index(unit: &CompilationUnit) -> Index {
    let mut indexer = SymbolIndexer::default();
    unit.walk(&mut indexer);
    indexer.index
}

#[derive(Default, Clone)]
struct Context {
    pub pou: String,
}

impl Context {
    pub fn replace_with_pou(&mut self, pou: impl Into<String>) -> Self {
        std::mem::replace(self, Context { pou: pou.into() })
    }
}

/// Indexer that registers all symbols in the index
#[derive(Default)]
pub struct SymbolIndexer {
    pub index: Index,
    ctx: Context,
}

/// The SymbolIndexer is responsible for registering all delcared types and symbols in the index.
impl AstVisitor for SymbolIndexer {
    /// Visits a VAR_GLOBAL VariableBlock and registers all variables as globals in the index
    fn visit_variable_block(&mut self, block: &plc_ast::ast::VariableBlock) {
        if block.kind == VariableBlockType::Global {
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
        let old_ctx = self.ctx.replace_with_pou(&pou.name);
        pou.properties.iter().for_each(|property| self.visit_property(property));
        self.ctx = old_ctx;
    }

    /// Visits an implementation and registers the implementation in the index
    fn visit_implementation(&mut self, implementation: &Implementation) {
        ImplementationIndexer::new(&mut self.index).index_implementation(implementation);
    }

    fn visit_config_variable(&mut self, config_variable: &plc_ast::ast::ConfigVariable) {
        self.index.config_variables.push(config_variable.clone());
    }

    fn visit_interface(&mut self, interface: &Interface) {
        for method in &interface.methods {
            self.visit_pou(method);

            // Register an implementation entry for each interface method so that codegen
            // can look up the function signature when generating indirect (itable) calls.
            // Interface methods have no body, but their LLVM function stubs are needed as
            // type templates for `build_indirect_call`.
            self.index.register_implementation(
                &method.name,
                &method.name,
                Some(&interface.ident.name.to_string()),
                ImplementationType::Method,
                false,
                method.location.clone(),
            );
        }

        self.index.interfaces.insert(interface.ident.name.to_owned(), InterfaceIndexEntry::from(interface));

        self.index.register_type(DataType {
            name: interface.ident.name.clone(),
            initial_value: None,
            information: DataTypeInformation::Interface { name: interface.ident.name.clone() },
            nature: TypeNature::Any,
            location: interface.ident.location.clone(),
        });
    }

    fn visit_property(&mut self, property: &PropertyBlock) {
        self.index.properties.insert(self.ctx.pou.clone(), property.ident.clone());
    }
}
