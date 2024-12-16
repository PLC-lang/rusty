//! Changes the calls to aggregate return types
//! to make them VAR_IN_OUT calls, allowing them
//! to be called from C_APIs and simplifying code generation

use std::{borrow::BorrowMut, fmt::Debug, sync::atomic::AtomicI32};

use plc_ast::{
    ast::{
        AccessModifier, Allocation, Assignment, AstFactory, AstNode, AstStatement, CallStatement,
        LinkageType, Pou, Variable, VariableBlock, VariableBlockType,
    },
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
    try_from_mut,
};
use plc_source::source_location::SourceLocation;

use crate::{
    index::{indexer, Index},
    resolver::AnnotationMap,
};

// Performs lowering for aggregate types defined in functions
pub struct AggregateTypeLowerer {
    index: Option<Index>,
    // New statements to be added during visit, should always be drained when read
    new_stmts: Vec<AstNode>,
    annotation: Option<Box<dyn AnnotationMap>>,
    id_provider: IdProvider,
    counter: AtomicI32,
}

impl AstVisitorMut for AggregateTypeLowerer {
    fn visit_compilation_unit(&mut self, unit: &mut plc_ast::ast::CompilationUnit) {
        if self.index.is_none() {
            //don't walk if we have no index to use
            return;
        }
        let old_index = indexer::index(unit);
        unit.walk(self);
        let new_index = indexer::index(unit);
        if let Some(index) = self.index.borrow_mut() {
            index.remove(old_index);
            index.import(new_index);
        }
    }
    // Change the signature for functions/methods with aggregate returns
    fn visit_pou(&mut self, pou: &mut Pou) {
        if pou.is_aggregate() {
            //Skip types that have already been made aggregates
            return;
        }
        let index = self.index.as_ref().expect("Can't get here without an index");
        //Check if pou has a return type
        if let Some(return_var) = pou.return_type.take() {
            let name = return_var.get_name().expect("We should have names at this point");
            let location = return_var.get_location();
            //Create a new return type for the pou
            pou.return_type.replace(plc_ast::ast::DataTypeDeclaration::Aggregate {
                referenced_type: name.to_string(),
                location,
            });
            let data_type = index.get_effective_type_or_void_by_name(name);
            if data_type.is_aggregate_type() {
                //Insert a new in out var to the pou variable block declarations
                let block = VariableBlock {
                    access: AccessModifier::Public,
                    constant: false,
                    retain: false,
                    variables: vec![Variable {
                        name: pou.get_return_name().to_string(),
                        data_type_declaration: return_var,
                        initializer: None,
                        address: None,
                        location: pou.name_location.clone(),
                    }],
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

    fn visit_implementation(&mut self, implementation: &mut plc_ast::ast::Implementation) {
        if self.annotation.is_none() {
            return;
        }
        let mut new_stmts = vec![];
        for stmt in implementation.statements.drain(..) {
            new_stmts.push(self.map(stmt));
        }
        implementation.statements.extend(new_stmts);
    }

    fn map(&mut self, mut node: AstNode) -> AstNode {
        node.borrow_mut().walk(self);
        let mut new_stmts = std::mem::take(&mut self.new_stmts);
        if new_stmts.is_empty() {
            node
        } else {
            let location = node.get_location();
            new_stmts.push(node);
            AstFactory::create_expression_list(new_stmts, location, self.id_provider.next_id())
        }
    }

    fn visit_assignment(&mut self, node: &mut AstNode) {
        let stmt = dbg!(try_from_mut!(node, Assignment).expect("Assignment"));
        stmt.walk(self);
        dbg!(&self.new_stmts);
    }

    fn visit_call_statement(&mut self, node: &mut AstNode) {
        let stmt = try_from_mut!(node, CallStatement).expect("CallStatement");
        dbg!(&self.new_stmts);
        dbg!(&stmt);
        stmt.walk(self);
        dbg!(&stmt);
        dbg!(&self.new_stmts);
        let annotation = self.annotation.as_ref().expect("Must be annotated");
        let index = self.index.as_ref().expect("Must be indexed");
        //Get the function being called
        let Some(crate::resolver::StatementAnnotation::Function { return_type: return_type_name, .. }) =
            annotation.get(&stmt.operator).or_else(|| annotation.get_hint(&stmt.operator))
        else {
            dbg!("Breaking");
            return;
        };
        let return_type = index.get_effective_type_or_void_by_name(return_type_name);
        //TODO: needs to be on the function
        if return_type.is_aggregate_type() {
            //self.context.insert_before(alloca)
            //TODO: use qualified name
            let name = format!(
                "__{}{}",
                stmt.operator.get_flat_reference_name().unwrap_or_default(),
                self.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            );
            //Create an allocation of the new type
            let alloca = AstNode {
                stmt: AstStatement::AllocationStatement(Allocation {
                    name: name.clone(),
                    reference_type: return_type_name.to_string(),
                }),
                id: self.id_provider.next_id(),
                location: SourceLocation::internal(),
            };
            self.new_stmts.push(alloca);
            let reference = super::create_member_reference(&name, self.id_provider.clone(), None);
            //TODO : we are creating th expression list twice in case of no params
            let AstNode { stmt: AstStatement::ExpressionList(mut parameters), id, location } =
                *stmt.parameters.take().unwrap_or_else(|| {
                    Box::new(AstFactory::create_expression_list(
                        vec![],
                        SourceLocation::internal(),
                        self.id_provider.next_id(),
                    ))
                })
            else {
                dbg!("Breaking");
                return;
            };
            parameters.insert(0, reference);

            stmt.parameters.replace(Box::new(AstFactory::create_expression_list(parameters, location, id)));
            //steal parameters, add one to the start, return parameters
            let mut reference = super::create_member_reference(&name, self.id_provider.clone(), None);
            std::mem::swap(node.get_stmt_mut(), reference.get_stmt_mut());
            self.new_stmts.push(dbg!(reference));
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use plc_ast::mut_visitor::AstVisitorMut;
    use plc_ast::provider::IdProvider;
    use pretty_assertions::assert_eq;

    use crate::lowering::calls::AggregateTypeLowerer;
    use crate::test_utils::tests::{annotate_with_ids, index as test_index, index_with_ids};

    #[test]
    fn function_with_simple_return_not_changed() {
        let (mut unit, index) = test_index(
            r#"
        FUNCTION simpleFunc : DINT
        VAR_INPUT
            x : DINT;
        END_VAR
        simpleFunc := 5;
        END_FUNCTION
        "#,
        );

        let (original_unit, _index) = test_index(
            r#"
        FUNCTION simpleFunc : DINT
        VAR_INPUT
            x : DINT;
        END_VAR
        simpleFunc := 5;
        END_FUNCTION
        "#,
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            new_stmts: Default::default(),
            id_provider: IdProvider::default(),
            counter: Default::default(),
        };
        lowerer.visit_compilation_unit(&mut unit);
        assert_eq!(unit, original_unit);
        assert_debug_snapshot!(lowerer.index.unwrap().find_pou_type("simpleFunc").unwrap());
    }

    #[test]
    fn function_with_string_return_is_changed() {
        let (mut unit, index) = test_index(
            r#"
        FUNCTION complexType : STRING
        VAR_INPUT
            x : DINT;
        END_VAR
        complexType := 'hello';
        END_FUNCTION
        "#,
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            new_stmts: Default::default(),
            id_provider: IdProvider::default(),
            counter: Default::default(),
        };
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.units[0]);
        assert_debug_snapshot!(lowerer.index.unwrap().find_pou_type("complexType").unwrap());
    }

    // Are we in a call?
    // foo(x:= baz()); callStatement -> Reference baz_1
    // foo(x:= baz()); callStatement -> Reference baz_2
    // foo(x:= baz()); callStatement -> Reference baz_3
    // foo(x:= baz()); callStatement -> Reference
    // foo(x:= baz()); callStatement -> Reference
    // foo(x:= baz()); callStatement -> Reference
    // foo(x:= baz()); callStatement -> Reference
    // alloca temp;
    // baz(temp);
    // foo(x := temp)
    //
    // call -> alloc, call, ref
    //
    // Insert alloca _before_ the call statement
    // x := foo();
    // alloca temp
    // foo(temp);
    // x := temp;
    //Check right, if a function call with aggregate, add allocation
    //fix call
    //assign to allocation
    //
    #[test]
    fn simple_call_statement() {
        let (mut unit, index) = test_index(
            r#"
        FUNCTION simpleFunc : DINT
        VAR_INPUT
            x : DINT;
        END_VAR
        simpleFunc := 5;
        END_FUNCTION

        FUNCTION main
            simpleFunc();
        END_FUNCTION
        "#,
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            new_stmts: Default::default(),
            id_provider: IdProvider::default(),
            counter: Default::default(),
        };
        lowerer.visit_compilation_unit(&mut unit);
        //Reparse the original unit without modifications
        let (original_unit, _index) = test_index(
            r#"
        FUNCTION simpleFunc : DINT
        VAR_INPUT
            x : DINT;
        END_VAR
        simpleFunc := 5;
        END_FUNCTION

        FUNCTION main
            simpleFunc();
        END_FUNCTION
        "#,
        );

        assert_eq!(unit, original_unit);
    }

    #[test]
    fn complex_call_statement_in_body() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION complexFunc : STRING
        VAR
            x : DINT;
        END_VAR
        complexFunc := 'hello';
        END_FUNCTION

        FUNCTION main
            // Should turn to
            // __alloca __complexFunc1 : STRING;
            // complexFunc(__complexFunc1);
            // __complexFunc1;
            complexFunc();
        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            new_stmts: Default::default(),
            id_provider: id_provider.clone(),
            counter: Default::default(),
        };
        lowerer.visit_compilation_unit(&mut unit);
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn complex_call_statement_in_assignment() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION complexFunc : STRING
        VAR
            x : DINT;
        END_VAR
        complexFunc := 'hello';
        END_FUNCTION

        FUNCTION main
        VAR a : STRING; END_VAR
            // Should turn to
            // __alloca __complexFunc1 : STRING;
            // complexFunc(__complexFunc1);
            // a := __complexFunc1;
            a := complexFunc();
        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            new_stmts: Default::default(),
            id_provider: id_provider.clone(),
            counter: Default::default(),
        };
        lowerer.visit_compilation_unit(&mut unit);
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn complex_call_statement_in_call() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION complexFunc : STRING
        VAR_INPUT
            x : STRING;
        END_VAR
        complexFunc := 'hello';
        END_FUNCTION

        FUNCTION main
        VAR a : STRING; END_VAR
            //Should be turned to:
            //alloca __complexFunc1 : STRING;
            //complexFunc(__complexFunc1, 'hello');
            //alloca __complexFunc2;
            //complexFunc(__complexFunc2, __complexFunc1);
            //a := __complexFunc2;
            a := complexFunc(x := complexFunc('hello'));
        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            new_stmts: Default::default(),
            id_provider: id_provider.clone(),
            counter: Default::default(),
        };
        lowerer.visit_compilation_unit(&mut unit);
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn complex_call_statement_in_assignment_twice() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION complexFunc : STRING
        VAR_INPUT
            x : DINT;
        END_VAR
        complexFunc := 'hello';
        END_FUNCTION

        FUNCTION main
        VAR a : STRING; END_VAR
            // Should turn to
            // __alloca __complexFunc1 : STRING;
            // complexFunc(__complexFunc1);
            // a := __complexFunc1;
            a := complexFunc();
            // Should turn to
            // __alloca __complexFunc2 : STRING;
            // complexFunc(__complexFunc2);
            // a := __complexFunc2;
            a := complexFunc();
        END_FUNCTION
        "#,
            id_provider.clone(),
        );

        let mut lowerer = AggregateTypeLowerer {
            index: Some(index),
            annotation: None,
            new_stmts: Default::default(),
            id_provider: id_provider.clone(),
            counter: Default::default(),
        };
        lowerer.visit_compilation_unit(&mut unit);
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }
}
