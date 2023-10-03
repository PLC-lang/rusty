use ast::{
    ast::{
        flatten_expression_list, Assignment, AstNode, AstStatement, CallStatement, CompilationUnit,
        LinkageType,
    },
    provider::IdProvider,
};
use insta::assert_debug_snapshot;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::SourceCode;

use crate::serializer2::{YBlock, YInOutVariable, YOutVariable, YReturn, YVariable};
use crate::{
    serializer2::{YInVariable, YPou},
    xml_parser::{self},
};

fn parse(content: &str) -> (CompilationUnit, Vec<Diagnostic>) {
    let source_code = SourceCode::new(content, "test.cfc");
    xml_parser::parse(&source_code, LinkageType::Internal, IdProvider::default())
}

#[test]
fn variable_assignment() {
    let content = YPou::init("foo", "program", "PROGRAM foo VAR a, b : DINT; END_VAR").with_fbd(vec![
        &YInVariable::new().with_id(1).with_expression("a"),
        &YOutVariable::new().with_id(2).with_execution_id(0).with_expression("b").connect_in(1),
    ]);

    let pou = xml_parser::visit(&content.serialize()).unwrap();
    assert_debug_snapshot!(pou);
}

#[test]
fn conditional_return() {
    let declaration = r#"
    FUNCTION_BLOCK conditional_return
        VAR_INPUT
            val : DINT;
        END_VAR
    "#;

    // TODO: Hmm, maybe replacing new() with id() wouldn't be so bad after all?
    // But then what about consistency, i.e. elements that do not have an id and instead have to be initialized with new()
    let content = YPou::init("conditional_return", "functionBlock", declaration).with_fbd(vec![
        &YInVariable::new().with_id(1).with_expression("val = 5"),
        &YReturn::new().with_id(2).with_execution_id(0).connect(1).negate(false),
        &YInVariable::new().with_id(3).with_expression("10"),
        &YOutVariable::new().with_id(4).with_execution_id(1).connect(3).with_expression("val"),
        &YInOutVariable::new().with_id(5).with_expression("a"),
    ]);

    let statements = &parse(&content.serialize()).0.implementations[0].statements;
    assert_eq!(statements.len(), 2);
    assert_debug_snapshot!(statements[0]);
}

#[test]
fn conditional_return_negated() {
    let declaration = r#"
    FUNCTION_BLOCK conditional_return
        VAR_INPUT
            val : DINT;
        END_VAR
    "#;

    let content = YPou::init("conditional_return", "functionBlock", declaration).with_fbd(vec![
        &YInVariable::new().with_id(1).with_expression("val = 5"),
        &YReturn::new().with_id(2).with_execution_id(0).negate(true).connect(1),
        &YInVariable::new().with_id(3).with_expression("10"),
        &YOutVariable::new().with_id(4).with_execution_id(1).connect(3).with_expression("val"),
        &YInOutVariable::new().with_id(5).with_expression("a"),
    ]);

    let statements = &parse(&content.serialize()).0.implementations[0].statements;

    assert_eq!(statements.len(), 2);
    assert_debug_snapshot!(statements[0]);
}

#[test]
fn conditional_return_without_connection() {
    let declaration = r#"
    FUNCTION_BLOCK conditional_return
        VAR_INPUT
            val : DINT;
        END_VAR
    "#;

    let content = YPou::init("conditional_return", "functionBlock", declaration).with_fbd(vec![
        &YInVariable::new().with_id(1).with_expression("val = 5"),
        &YReturn::new().with_id(2).with_execution_id(0).negate(false), // This return isn't connected to any other node
        &YInVariable::new().with_id(3).with_expression("10"),
        &YOutVariable::new().with_id(4).with_execution_id(1).with_expression("val").connect(3),
        &YInOutVariable::new().with_id(5).with_expression("a"),
    ]);

    let (_, diagnostics) = parse(&content.serialize());
    assert_eq!(diagnostics.len(), 1);
    assert_debug_snapshot!(diagnostics);
}

#[test]
fn conditional_return_chained_to_another_conditional_return() {
    let declaration = r#"
    FUNCTION_BLOCK conditional_return
        VAR_INPUT
            val : DINT;
        END_VAR
    "#;

    let content = YPou::init("conditional_return", "functionBlock", declaration).with_fbd(vec![
        &YReturn::new().with_id(1).with_execution_id(0),
        &YReturn::new().with_id(2).with_execution_id(1).connect(1),
    ]);

    let (_, diagnostics) = parse(&content.serialize());
    assert_eq!(diagnostics.len(), 2);
    assert_debug_snapshot!(diagnostics);
}

#[test]
fn model_is_sorted_by_execution_order() {
    let content = YPou::init("foo", "program", "PROGRAM foo VAR a, b, c, d : DINT; END_VAR").with_fbd(vec![
        &YInVariable::new().with_id(1).with_expression("a"),
        &YOutVariable::new().with_id(2).with_execution_id(2).with_expression("b").connect(1),
        &YOutVariable::new().with_id(3).with_execution_id(0).with_expression("c").connect(1),
        &YOutVariable::new().with_id(4).with_execution_id(1).with_expression("d").connect(1),
    ]);

    assert_debug_snapshot!(xml_parser::visit(&content.serialize()).unwrap());
}

#[test]
fn function_returns() {
    let content =
        YPou::init("foo", "function", "FUNCTION foo : DINT VAR_INPUT a : DINT; END_VAR").with_fbd(vec![
            &YInVariable::new().with_id("1").with_expression("a"),
            &YOutVariable::new().with_id("2").with_execution_id(0).with_expression("foo").connect(1),
        ]);

    assert_debug_snapshot!(xml_parser::visit(&content.serialize()).unwrap());
}

#[test]
fn ast_generates_locations() {
    let content = YPou::init("foo", "program", "PROGRAM foo VAR a, x : DINT; END_VAR").with_fbd(vec![
        &YInVariable::new().with_id(1).with_expression("x"),
        &YOutVariable::new().with_id(2).with_expression("a").with_execution_id(0).connect(1),
        &YBlock::init("ADD", 3, 1)
            .with_input_variables(vec![
                &YVariable::new().with_name("").connect(4),
                &YVariable::new().with_name("").connect(5),
            ])
            .with_output_variables(vec![&YVariable::new().with_name("")])
            .with_inout_variables(vec![]),
        &YInVariable::new().with_id(4).with_expression("a"),
        &YInVariable::new().with_id(5).with_expression("1"),
    ]);

    let source_code = SourceCode::new(&content.serialize(), "<internal>.cfc");
    let (units, diagnostics) = xml_parser::parse(&source_code, LinkageType::Internal, IdProvider::default());
    let impl1 = &units.implementations[0];
    //Deconstruct assignment and get locations
    let AstStatement::Assignment(Assignment { left, right, .. }) = &impl1.statements[0].get_stmt() else {
        panic!("Not an assignment");
    };
    assert_debug_snapshot!(left.get_location());
    assert_debug_snapshot!(right.get_location());
    //Deconstruct call statement and get locations
    let AstNode {
        stmt: AstStatement::CallStatement(CallStatement { operator, parameters, .. }),
        location,
        ..
    } = &impl1.statements[1]
    else {
        panic!("Not a call statement");
    };
    assert_debug_snapshot!(location);
    assert_debug_snapshot!(operator.get_location());
    let parameters = parameters.as_deref().unwrap();
    let parameters = flatten_expression_list(parameters);
    for param in parameters {
        assert_debug_snapshot!(param.get_location());
    }

    assert_debug_snapshot!(impl1);
    assert!(diagnostics.is_empty());
}

#[test]
#[ignore = "Validation is not implemented on CFC tests yet, we need to be able to change parsers on the test utils level"]
fn ast_diagnostic_locations() {
    let content = YPou::init("foo", "program", "PROGRAM foo VAR x : DINT; END_VAR").with_fbd(vec![
        &YInVariable::new().with_id(1).with_expression("x"),
        &YOutVariable::new().with_id(2).with_execution_id(0).with_expression("a").connect(1), // "a" isn't declared anywhere, hence the error
    ]);

    let source_code = SourceCode::new(&content.serialize(), "<internal>.cfc");
    let (units, diagnostics) = xml_parser::parse(&source_code, LinkageType::Internal, IdProvider::default());
    let impl1 = &units.implementations[0];
    assert_debug_snapshot!(impl1);
    assert!(diagnostics.is_empty());
    //Run resolve and validate
    todo!("Validation in tests not yet done")
}
