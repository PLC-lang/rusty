//! Changes the calls to aggregate return types
//! to make them VAR_IN_OUT calls, allowing them
//! to be called from C_APIs and simplifying code generation

use std::{borrow::BorrowMut, sync::atomic::AtomicI32};

use plc_ast::{
    ast::{
        steal_expression_list, AccessModifier, Allocation, Assignment, AstFactory, AstNode, AstStatement,
        CallStatement, CompilationUnit, LinkageType, Pou, Variable, VariableBlock, VariableBlockType,
    },
    control_statements::{AstControlStatement, ConditionalBlock},
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
    try_from_mut,
};
use plc_source::source_location::SourceLocation;

use crate::{index::Index, resolver::AnnotationMap};

// Performs lowering for aggregate types defined in functions
#[derive(Default)]
pub struct AggregateTypeLowerer {
    pub index: Option<Index>,
    pub annotation: Option<Box<dyn AnnotationMap>>,
    pub id_provider: IdProvider,
    // New statements to be added during visit, should always be drained when read
    new_stmts: Vec<AstNode>,
    counter: AtomicI32,
}

impl AggregateTypeLowerer {
    pub fn new(id_provider: IdProvider) -> Self {
        Self { id_provider, ..Default::default() }
    }

    pub fn visit(&mut self, units: &mut [CompilationUnit]) {
        units.iter_mut().for_each(|u| self.visit_compilation_unit(u));
    }

    pub fn visit_unit(&mut self, unit: &mut CompilationUnit) {
        self.visit_compilation_unit(unit);
    }

    fn steal_and_walk_list(&mut self, list: &mut Vec<AstNode>) {
        let mut new_stmts = vec![];
        for stmt in list.drain(..) {
            new_stmts.push(self.map(stmt));
        }
        std::mem::swap(list, &mut new_stmts);
    }

    fn walk_conditional_blocks(&mut self, blocks: &mut Vec<ConditionalBlock>) {
        for b in blocks {
            b.condition.walk(self);
            self.steal_and_walk_list(&mut b.body);
        }
    }
}

impl AstVisitorMut for AggregateTypeLowerer {
    fn visit_compilation_unit(&mut self, unit: &mut plc_ast::ast::CompilationUnit) {
        if self.index.is_none() {
            //don't walk if we have no index to use
            return;
        }
        unit.walk(self);
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
        let stmt = try_from_mut!(node, Assignment).expect("Assignment");
        stmt.walk(self);
    }

    fn visit_call_statement(&mut self, node: &mut AstNode) {
        let original_location = node.get_location();
        // self.steal_and_walk_call_statement(node);
        let stmt = try_from_mut!(node, CallStatement).expect("CallStatement");
        stmt.walk(self);
        let Some((annotation, index)) = self.annotation.as_ref().zip(self.index.as_ref()) else {
            //Early exit if not annotated or indexed
            return;
        };
        //Get the function being called
        let Some(crate::resolver::StatementAnnotation::Function {
            qualified_name,
            return_type: return_type_name,
            ..
        }) = annotation.get(&stmt.operator).or_else(|| annotation.get_hint(&stmt.operator))
        else {
            return;
        };
        let function_entry = index.find_pou(qualified_name).expect("Function not found");
        let return_type = index.get_effective_type_or_void_by_name(return_type_name);
        //TODO: needs to be on the function
        if return_type.is_aggregate_type() && !function_entry.is_buitin() {
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
                location: original_location.clone(),
            };
            self.new_stmts.push(alloca);
            let location = stmt.parameters.as_ref().map(|it| it.get_location()).unwrap_or_default();
            let id = stmt.parameters.as_ref().map(|it| it.get_id()).unwrap_or(self.id_provider.next_id());
            let reference = super::create_member_reference_with_location(
                &name,
                self.id_provider.clone(),
                None,
                original_location.clone(),
            );
            //TODO : we are creating th expression list twice in case of no params
            let mut parameters =
                stmt.parameters.as_mut().map(|it| steal_expression_list(it.borrow_mut())).unwrap_or_default();

            parameters.insert(0, reference);

            stmt.parameters.replace(Box::new(AstFactory::create_expression_list(parameters, location, id)));
            //steal parameters, add one to the start, return parameters
            let mut reference = super::create_member_reference_with_location(
                &name,
                self.id_provider.clone(),
                None,
                original_location,
            );
            std::mem::swap(node.get_stmt_mut(), reference.get_stmt_mut());
            self.new_stmts.push(reference);
        }
    }

    fn visit_control_statement(&mut self, node: &mut AstNode) {
        let ctrl_stmt = try_from_mut!(node, AstControlStatement).expect("ControlStatement");
        match ctrl_stmt {
            AstControlStatement::If(stmt) => {
                self.walk_conditional_blocks(&mut stmt.blocks);
                self.steal_and_walk_list(&mut stmt.else_block);
            }
            AstControlStatement::WhileLoop(stmt) | AstControlStatement::RepeatLoop(stmt) => {
                stmt.condition.walk(self);
                self.steal_and_walk_list(&mut stmt.body);
            }
            AstControlStatement::ForLoop(stmt) => {
                stmt.counter.walk(self);
                stmt.start.walk(self);
                stmt.end.walk(self);
                if let Some(ref mut step) = stmt.by_step {
                    step.walk(self);
                }
                self.steal_and_walk_list(&mut stmt.body);
            }
            AstControlStatement::Case(stmt) => {
                stmt.selector.walk(self);
                self.walk_conditional_blocks(&mut stmt.case_blocks);
                self.steal_and_walk_list(&mut stmt.else_block);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use plc_ast::mut_visitor::AstVisitorMut;
    use plc_ast::provider::IdProvider;
    use pretty_assertions::assert_eq;

    use crate::index::indexer;
    use crate::lowering::calls::AggregateTypeLowerer;
    use crate::test_utils::tests::{
        annotate_and_lower_with_ids, annotate_with_ids, index as test_index, index_and_lower,
        index_unit_with_id, index_with_ids,
    };

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
        lowerer.index.replace(indexer::index(&unit));
        assert_eq!(unit, original_unit);
        assert_debug_snapshot!(lowerer.index.unwrap().find_pou_type("simpleFunc").unwrap());
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
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        assert_debug_snapshot!(unit.units[0]);
        assert_debug_snapshot!(lowerer.index.unwrap().find_pou_type("complexType").unwrap());
    }

    #[test]
    fn method_with_string_return_is_changed() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION_BLOCK fb
        METHOD complexMethod : STRING
            complexMethod := 'hello';
        END_METHOD
        END_FUNCTION_BLOCK
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
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        assert_debug_snapshot!(unit.units[1]);
        assert_debug_snapshot!(lowerer.index.unwrap().find_pou_type("fb.complexMethod").unwrap());
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
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
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
        //re-index the new unit
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
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
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn complex_call_statement_in_assignment_method() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION_BLOCK fb
        METHOD complexMethod : STRING
            complexMethod := 'hello';
        END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
        VAR
            a : STRING;
            myFb : fb;
        END_VAR
            // Should turn to
            // __alloca __complexFunc1 : STRING;
            // complexFunc(__complexFunc1);
            // a := __complexFunc1;
            a := myFb.complexMethod();
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
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[2]);
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
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
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
            a := complexFunc(x := complexFunc(x := 'hello'));
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
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn complex_call_statement_in_call_with_implicit_parameter() {
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
        VAR a, b : STRING; END_VAR
            //Should be turned to:
            //alloca __complexFunc1 : STRING;
            //complexFunc(__complexFunc1, b);
            //alloca __complexFunc2;
            //complexFunc(__complexFunc2, __complexFunc1);
            //a := __complexFunc2;
            a := complexFunc(x := complexFunc(b));
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
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn complex_call_statement_in_call_with_implicit_literal_parameter() {
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
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
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
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn complex_call_statement_in_if_statement_body() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION complexFunc : STRING
        complexFunc := 'hello';
        END_FUNCTION

        FUNCTION main
        VAR a : STRING; END_VAR
        IF TRUE THEN
            //Should be turned to:
            //alloca __complexFunc1 : STRING;
            //complexFunc(__complexFunc1);
            //a := __complexFunc1;
            a := complexFunc();
        END_IF

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
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn complex_call_statement_in_if_statement_condition() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION complexFunc : STRING
        complexFunc := 'hello';
        END_FUNCTION

        FUNCTION main
        VAR a : STRING; END_VAR
        //Should be turned to:
        //alloca __complexFunc1 : STRING;
        //complexFunc(__complexFunc1);
        //IF __complexFunc1 = 'hello; THEN ... END_IF
        IF complexFunc() = 'hello' THEN
            // do nothing
        END_IF

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
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn complex_call_statement_in_else_block() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION complexFunc : STRING
        complexFunc := 'hello';
        END_FUNCTION

        FUNCTION main
        VAR a : STRING; END_VAR
        IF FALSE THEN
            // do nothing
        ELSE
            //Should be turned to:
            //alloca __complexFunc1 : STRING;
            //complexFunc(__complexFunc1);
            //a := __complexFunc1;
            a := complexFunc();
        END_IF

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
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn complex_call_statement_in_elif_condition() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION complexFunc : STRING
        complexFunc := 'hello';
        END_FUNCTION

        FUNCTION main
        VAR a : STRING; END_VAR
        IF TRUE THEN
            // do nothing
        ELSIF complexFunc() = 'hello' THEN // FIXME: currently has side-effects, is always evaluated
            // do nothing
        END_IF

        END_FUNCTION

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
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[1]);
    }

    #[test]
    fn do_not_change_builtin_call() {
        let id_provider = IdProvider::default();
        let (mut unit, index) = index_with_ids(
            r#"
        FUNCTION main
        VAR a : STRING; END_VAR
            a := SEL('hello', 'world');
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
        lowerer.index.replace(index_unit_with_id(&unit, id_provider.clone()));
        let annotations = annotate_with_ids(&unit, lowerer.index.as_mut().unwrap(), id_provider.clone());
        lowerer.annotation.replace(Box::new(annotations));
        lowerer.visit_compilation_unit(&mut unit);
        assert_debug_snapshot!(unit.implementations[0]);
    }

    #[test]
    fn call_statements_in_initializers_not_changed() {
        let id_provider = IdProvider::default();
        let src = r#"
        FUNCTION main
        VAR
            a : STRING;
            b : REF_TO STRING := REF(a);
            c : REFERENCE TO STRING REF=b;
            b : POINTER TO STRING := ADR(a);
        END_VAR
        END_FUNCTION
        "#;

        let (unit, index, ..) = index_and_lower(src, id_provider.clone());
        let (_, _, units) = annotate_and_lower_with_ids(unit, index, id_provider.clone());
        assert_debug_snapshot!(units[0].0.implementations[0]);
    }

    #[test]
    fn call_statemements_in_global() {
        let id_provider = IdProvider::default();
        let src = r#"
        VAR_GLOBAL
            a : STRING;
            b : REF_TO STRING := REF(a);
            c : REFERENCE TO STRING REF=b;
            b : POINTER TO STRING := ADR(a);
        END_VAR
        "#;

        let (unit, index, ..) = index_and_lower(src, id_provider.clone());
        let (_, _, units) = annotate_and_lower_with_ids(unit, index, id_provider.clone());
        assert_debug_snapshot!(units[0].0.global_vars);
    }

    #[test]
    fn generic_call_statement() {

        let id_provider = IdProvider::default();
        let src = r#"
        FUNCTION main : STRING
            main := MID('hello');
        END_FUNCTION

        {external}
        FUNCTION MID < T: ANY_STRING >: T
        VAR_INPUT
            IN: T;
        END_VAR
        END_FUNCTION
        "#;

        let (unit, index, ..) = index_and_lower(src, id_provider.clone());
        let (_, index, units) = annotate_and_lower_with_ids(unit, index, id_provider.clone());
        assert_debug_snapshot!(index.find_pou_type("MID__STRING").unwrap());
        assert_debug_snapshot!(units[0].0.implementations[1]);
    }
}
