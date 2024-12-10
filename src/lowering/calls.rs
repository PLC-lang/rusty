//! Changes the calls to aggregate return types
//! to make them VAR_IN_OUT calls, allowing them
//! to be called from C_APIs and simplifying code generation

use plc_ast::{ast::{AccessModifier, LinkageType, Pou, PouType, Variable, VariableBlock, VariableBlockType}, mut_visitor::{AstVisitorMut, WalkerMut}, visitor};
use plc_source::source_location::SourceLocation;

use crate::index::{indexer, Index};

pub struct AggregateTypeLowerer {
    index : Index,
}

impl AstVisitorMut for AggregateTypeLowerer {

    fn visit_compilation_unit(&mut self, unit: &mut plc_ast::ast::CompilationUnit) {
        let old_index = indexer::index(unit);
        unit.walk(self);
        let new_index = indexer::index(unit);
        self.index.remove(old_index);
        self.index.import(new_index);

    }
    // Change the signature for functions/methods with aggregate returns
    fn visit_pou(&mut self, pou: &mut Pou) {
        //Check if pou has a return type
        if let Some(return_var) = pou.return_type.take() {
            let name = return_var.get_name().expect("We should have names at this point");
            let data_type = self.index.get_effective_type_or_void_by_name(name);
            if data_type.is_aggregate_type() {
                //Insert a new in out var to the pou variable block declarations
                let block = VariableBlock {
                    access: AccessModifier::Public,
                    constant: false,
                    retain: false,
                    variables: vec![
                        Variable {
                                name: pou.get_return_name().to_string(),
                                data_type_declaration: return_var,
                                initializer: None,
                                address: None,
                                location: pou.name_location.clone() }
                    ],
                    variable_block_type: VariableBlockType::InOut,
                    linkage: LinkageType::Internal,
                    location: SourceLocation::internal(),
                };
                pou.variable_blocks.insert(0, block)
            } else {
                pou.return_type.replace(return_var);
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use plc_ast::{mut_visitor::AstVisitorMut, provider::IdProvider};

    use crate::{lowering::calls::AggregateTypeLowerer, test_utils::tests::index_with_ids};

    #[test]
    fn function_with_simple_return_not_changed() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
        r#"
        FUNCTION simpleFunc : DINT
        VAR_INPUT
            x : DINT;
        END_VAR
        simpleFunc := 5;
        END_FUNCTION
        "#
        , id_provider.clone());

        let mut lowerer = AggregateTypeLowerer {
            index
        };
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit);
        assert_debug_snapshot!(lowerer.index.find_pou_type("simpleFunc").unwrap());
    }

    #[test]
    fn function_with_string_return_is_changed() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
        r#"
        FUNCTION complexType : STRING
        VAR_INPUT
            x : DINT;
        END_VAR
        complexType := 'hello';
        END_FUNCTION
        "#
        , id_provider.clone());

        let mut lowerer = AggregateTypeLowerer {
            index
        };
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit);
        assert_debug_snapshot!(lowerer.index.find_pou_type("complexType").unwrap());
    }
}
