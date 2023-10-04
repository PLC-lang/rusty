use ast::{
    ast::{
        flatten_expression_list, Assignment, AstNode, AstStatement, CallStatement, CompilationUnit,
        LinkageType,
    },
    provider::IdProvider,
};
use insta::assert_debug_snapshot;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::{source_location::SourceLocationFactory, SourceCode, SourceCodeFactory};

use crate::serializer::{
    YBlock, YConnector, YContinuation, YInOutVariable, YInVariable, YOutVariable, YPou, YReturn, YVariable,
};
use crate::{
    model::project::Project,
    xml_parser::{self},
};

fn parse(content: &str) -> (CompilationUnit, Vec<Diagnostic>) {
    let source_code = SourceCode::new(content, "test.cfc");
    xml_parser::parse(&source_code, LinkageType::Internal, IdProvider::default())
}

fn visit(content: &str) -> Result<Project, crate::error::Error> {
    xml_parser::visit(content)
}

fn visit_and_desugar(content: &str) -> Result<Project, Vec<Diagnostic>> {
    let Ok(mut project) = visit(content) else { unreachable!() };
    let source_location_factory = SourceLocationFactory::for_source(&content.create_source("test"));
    project.desugar(&source_location_factory)?;
    Ok(project)
}

#[test]
fn variable_assignment() {
    let content = YPou::init("foo", "program", "PROGRAM foo VAR a, b : DINT; END_VAR")
        .with_fbd(vec![
            &YInVariable::id(1).with_expression("a"),
            &YOutVariable::id(2).with_execution_id(0).with_expression("b").connect_in(1),
        ])
        .serialize();

    let pou = xml_parser::visit(&content).unwrap();
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

    let content = YPou::init("conditional_return", "functionBlock", declaration).with_fbd(vec![
        &YInVariable::id(1).with_expression("val = 5"),
        &YReturn::id(2).with_execution_id(0).connect(1).negate(false),
        &YInVariable::id(3).with_expression("10"),
        &YOutVariable::id(4).with_execution_id(1).connect(3).with_expression("val"),
        &YInOutVariable::id(5).with_expression("a"),
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
        &YInVariable::id(1).with_expression("val = 5"),
        &YReturn::id(2).with_execution_id(0).negate(true).connect(1),
        &YInVariable::id(3).with_expression("10"),
        &YOutVariable::id(4).with_execution_id(1).connect(3).with_expression("val"),
        &YInOutVariable::id(5).with_expression("a"),
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
        &YInVariable::id(1).with_expression("val = 5"),
        &YReturn::id(2).with_execution_id(0).negate(false), // This return isn't connected to any other node
        &YInVariable::id(3).with_expression("10"),
        &YOutVariable::id(4).with_execution_id(1).with_expression("val").connect(3),
        &YInOutVariable::id(5).with_expression("a"),
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
        &YReturn::id(1).with_execution_id(0),
        &YReturn::id(2).with_execution_id(1).connect(1),
    ]);

    let (_, diagnostics) = parse(&content.serialize());
    assert_eq!(diagnostics.len(), 2);
    assert_debug_snapshot!(diagnostics);
}

#[test]
fn model_is_sorted_by_execution_order() {
    let content = YPou::init("foo", "program", "PROGRAM foo VAR a, b, c, d : DINT; END_VAR").with_fbd(vec![
        &YInVariable::id(1).with_expression("a"),
        &YOutVariable::id(2).with_execution_id(2).with_expression("b").connect(1),
        &YOutVariable::id(3).with_execution_id(0).with_expression("c").connect(1),
        &YOutVariable::id(4).with_execution_id(1).with_expression("d").connect(1),
    ]);

    assert_debug_snapshot!(xml_parser::visit(&content.serialize()).unwrap());
}

#[test]
fn connection_variable_source_to_multiple_sinks_parses() {
    let declaration = r#"
        FUNCTION myConnection : DINT
        VAR_INPUT
            x: DINT;
        END_VAR
        VAR_TEMP
            y: DINT;
        END_VAR
    "#;

    #[rustfmt::skip]
    let content = YPou::init("myConnection", "function", declaration).with_fbd(vec![
        &YConnector::id(1).with_name("s1").connect_in(2),
        &YContinuation::id(3).with_name("s1"),
        &YInVariable::id(2).with_expression("x"),
        &YOutVariable::id(4).with_expression("myConnection").with_execution_id(2).connect_temp(9, "myAdd"),
        &YInVariable::id(7).with_expression("y"),
        &YOutVariable::id(8).with_expression("y").with_execution_id(0).connect(3),
        &YBlock::init("myAdd", 9, 1)
            .with_input_variables(vec![
                &YVariable::new().with_name("a").connect(7),
                &YVariable::new().with_name("b").connect(3),
            ])
            .with_output_variables(vec![&YVariable::new().with_name("myAdd")]),
    ]).serialize();

    assert_debug_snapshot!(parse(&content).0.implementations[0].statements);
}

#[test]
#[ignore = "block-to-block connections not yet implemented"]
fn connection_block_source_to_multiple_sinks_parses() {
    assert_debug_snapshot!(parse(content::BLOCK_SOURCE_TO_MULTI_SINK).0.implementations[0].statements);
}

#[test]
fn direct_connection_of_sink_to_other_source_generates_correct_model() {
    let declaration = r#"
        FUNCTION myConnection : DINT
        VAR_INPUT
            x: DINT;
        END_VAR
    "#;

    let content = YPou::init("myConnection", "function", declaration).with_fbd(vec![
        &YConnector::id(1).with_name("s1").connect_in(16),
        &YContinuation::id(3).with_name("s1"),
        &YOutVariable::id(4).with_expression("myConnection").with_execution_id(3).connect(20),
        &YInVariable::id(16).with_expression("x"),
        &YConnector::id(21).with_name("s2").connect_in(3),
        &YContinuation::id(20).with_name("s2"),
    ]);

    assert_debug_snapshot!(visit_and_desugar(&content.serialize()).unwrap());
}

#[test]
fn direct_connection_of_sink_to_other_source_ast_parses() {
    let declaration = r#"
        FUNCTION myConnection : DINT
        VAR_INPUT
            x: DINT;
        END_VAR
    "#;

    let content = YPou::init("myConnection", "function", declaration).with_fbd(vec![
        &YConnector::id(1).with_name("s1").connect_in(16),
        &YContinuation::id(3).with_name("s1"),
        &YOutVariable::id(4).with_expression("myConnection").with_execution_id(3).connect(20),
        &YInVariable::id(16).with_expression("x"),
        &YConnector::id(21).with_name("s2").connect_in(3),
        &YContinuation::id(20).with_name("s2"),
    ]);

    assert_debug_snapshot!(parse(&content.serialize()).0.implementations[0].statements);
}

#[test]
fn return_connected_to_sink_parses() {
    let declaration = "FUNCTION positivOrZero : DINT VAR_INPUT x : DINT; END_VAR";
    #[rustfmt::skip]
    let content = YPou::init("positiveOrZero", "function", declaration).with_fbd(vec![
        &YConnector::id(1).with_name("s1").connect_in(2),
        &YContinuation::id(3).with_name("s1"),
        &YConnector::id(4).with_name("s2").connect_in(3),
        &YContinuation::id(5).with_name("s2"),
        &YReturn::id(6).with_execution_id(0).connect(5),
        &YInVariable::id(2).with_expression("x &lt; 0"), // TODO: The less-than symbol has to be written this way?
        &YOutVariable::id(7).with_execution_id(1).with_expression("positiveOrZero").connect(8),
        &YInVariable::id(8).with_expression("x"),
    ]);

    assert_debug_snapshot!(parse(&content.serialize()).0.implementations[0].statements);
}

#[test]
fn sink_source_data_recursion_does_not_overflow_the_stack() {
    let declaration = "FUNCTION myConnection : DINT VAR_INPUT x: DINT; END_VAR";
    let content = YPou::init("myConnection", "function", declaration).with_fbd(vec![
        &YConnector::id(22).with_name("s1").connect_in(23),
        &YContinuation::id(24).with_name("s1"),
        &YConnector::id(25).with_name("s2").connect_in(24),
        &YContinuation::id(26).with_name("s2"),
        &YConnector::id(27).with_name("s3").connect_in(26),
        &YContinuation::id(23).with_name("s3"),
    ]);

    let Err(diagnostics) = visit_and_desugar(&content.serialize()) else {
        panic!("Expected test to report data recursion!")
    };
    assert_debug_snapshot!(diagnostics);
}

#[test]
fn unconnected_connections() {
    let declaration = "FUNCTION unconnectedConnections : DINT VAR_INPUT x : DINT; END_VAR";
    let content = YPou::init("unconnectedConnections", "function", declaration).with_fbd(vec![
        &YConnector::id(1).with_name("s1"),
        &YContinuation::id(2).with_name("s1"),
        &YConnector::id(3).with_name("s2").connect_in(2),
        &YContinuation::id(4).with_name("s2"),
    ]);

    let Err(diagnostics) = visit_and_desugar(&content.serialize()) else {
        panic!("Expected test to report unconnected source!")
    };
    assert_debug_snapshot!(diagnostics);
}

#[test]
fn unassociated_connections() {
    let declaration = "FUNCTION unconnectedConnections : DINT VAR_INPUT x : DINT; END_VAR";
    let content = YPou::init("unassociatedSink", "function", declaration).with_fbd(vec![
        &YConnector::id(1).with_name("s1").connect_in(2),
        &YContinuation::id(3).with_name("s2"),
        &YInVariable::id(2).with_expression("x"),
        &YOutVariable::id(4).with_expression("unassociatedSink").with_execution_id(0).connect(3),
    ]);

    let Err(diagnostics) = visit_and_desugar(&content.serialize()) else {
        panic!("Expected test to report unassociated sink!")
    };
    assert_debug_snapshot!(diagnostics);
}

#[test]
fn function_returns() {
    let content =
        YPou::init("foo", "function", "FUNCTION foo : DINT VAR_INPUT a : DINT; END_VAR").with_fbd(vec![
            &YInVariable::id(1).with_expression("a"),
            &YOutVariable::id(2).with_execution_id(0).with_expression("foo").connect(1),
        ]);

    assert_debug_snapshot!(xml_parser::visit(&content.serialize()).unwrap());
}

#[test]
fn ast_generates_locations() {
    let content = YPou::init("foo", "program", "PROGRAM foo VAR a, x : DINT; END_VAR").with_fbd(vec![
        &YInVariable::id(1).with_expression("x"),
        &YOutVariable::id(2).with_expression("a").with_execution_id(0).connect(1),
        &YBlock::init("ADD", 3, 1)
            .with_input_variables(vec![
                &YVariable::new().with_name("").connect(4),
                &YVariable::new().with_name("").connect(5),
            ])
            .with_output_variables(vec![&YVariable::new().with_name("")])
            .with_inout_variables(vec![]),
        &YInVariable::id(4).with_expression("a"),
        &YInVariable::id(5).with_expression("1"),
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
        &YInVariable::id(1).with_expression("x"),
        &YOutVariable::id(2).with_execution_id(0).with_expression("a").connect(1), // "a" isn't declared anywhere, hence the error
    ]);

    let source_code = SourceCode::new(&content.serialize(), "<internal>.cfc");
    let (units, diagnostics) = xml_parser::parse(&source_code, LinkageType::Internal, IdProvider::default());
    let impl1 = &units.implementations[0];
    assert_debug_snapshot!(impl1);
    assert!(diagnostics.is_empty());
    //Run resolve and validate
    todo!("Validation in tests not yet done")
}

mod content {
    pub(super) const BLOCK_SOURCE_TO_MULTI_SINK: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
    <pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="myConnection" pouType="function">
        <interface>
            <localVars/>
            <addData>
                <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                    <textDeclaration>
                        <content>
    FUNCTION myConnection : DINT
    VAR_INPUT
        x: DINT;
    END_VAR
    VAR_TEMP
        y: DINT;
    END_VAR
                </content>
                    </textDeclaration>
                </data>
            </addData>
        </interface>
        <body>
            <FBD>
                <connector name="s1" localId="1" height="20" width="54">
                    <position x="500" y="190"/>
                    <connectionPointIn>
                        <relPosition x="0" y="10"/>
                        <connection refLocalId="14" formalParameter="myAdd"/>
                    </connectionPointIn>
                </connector>
                <continuation name="s1" localId="3" height="20" width="64">
                    <position x="620" y="210"/>
                    <connectionPointOut>
                        <relPosition x="64" y="10"/>
                    </connectionPointOut>
                </continuation>
                <outVariable localId="4" height="20" width="124" executionOrderId="3" negated="false" storage="none">
                    <position x="1030" y="190"/>
                    <connectionPointIn>
                        <relPosition x="0" y="10"/>
                        <connection refLocalId="15" formalParameter="myAdd"/>
                    </connectionPointIn>
                    <expression>myConnection</expression>
                </outVariable>
                <block localId="14" width="80" height="60" typeName="myAdd" executionOrderId="0">
                    <position x="300" y="170"/>
                    <inputVariables>
                        <variable formalParameter="a" negated="false">
                            <connectionPointIn>
                                <relPosition x="0" y="30"/>
                                <connection refLocalId="16"/>
                            </connectionPointIn>
                        </variable>
                        <variable formalParameter="b" negated="false">
                            <connectionPointIn>
                                <relPosition x="0" y="50"/>
                                <connection refLocalId="17"/>
                            </connectionPointIn>
                        </variable>
                    </inputVariables>
                    <inOutVariables/>
                    <outputVariables>
                        <variable formalParameter="myAdd" negated="false">
                            <connectionPointOut>
                                <relPosition x="80" y="30"/>
                            </connectionPointOut>
                        </variable>
                    </outputVariables>
                </block>
                <block localId="15" width="80" height="60" typeName="myAdd" executionOrderId="2">
                    <position x="900" y="170"/>
                    <inputVariables>
                        <variable formalParameter="a" negated="false">
                            <connectionPointIn>
                                <relPosition x="0" y="30"/>
                                <connection refLocalId="18"/>
                            </connectionPointIn>
                        </variable>
                        <variable formalParameter="b" negated="false">
                            <connectionPointIn>
                                <relPosition x="0" y="50"/>
                                <connection refLocalId="3"/>
                            </connectionPointIn>
                        </variable>
                    </inputVariables>
                    <inOutVariables/>
                    <outputVariables>
                        <variable formalParameter="myAdd" negated="false">
                            <connectionPointOut>
                                <relPosition x="80" y="30"/>
                            </connectionPointOut>
                        </variable>
                    </outputVariables>
                </block>
                <inVariable localId="16" height="20" width="80" negated="false">
                    <position x="150" y="190"/>
                    <connectionPointOut>
                        <relPosition x="80" y="10"/>
                    </connectionPointOut>
                    <expression>x</expression>
                </inVariable>
                <inVariable localId="17" height="20" width="80" negated="false">
                    <position x="150" y="210"/>
                    <connectionPointOut>
                        <relPosition x="80" y="10"/>
                    </connectionPointOut>
                    <expression>y</expression>
                </inVariable>
                <inVariable localId="18" height="20" width="80" negated="false">
                    <position x="810" y="190"/>
                    <connectionPointOut>
                        <relPosition x="80" y="10"/>
                    </connectionPointOut>
                    <expression>y</expression>
                </inVariable>
                <outVariable localId="19" height="20" width="80" executionOrderId="1" negated="false" storage="none">
                    <position x="700" y="190"/>
                    <connectionPointIn>
                        <relPosition x="0" y="10"/>
                        <connection refLocalId="3">
                            <position x="700" y="200"/>
                            <position x="690" y="200"/>
                            <position x="690" y="220"/>
                            <position x="684" y="220"/>
                        </connection>
                    </connectionPointIn>
                    <expression>y</expression>
                </outVariable>
            </FBD>
        </body>
    </pou>
    "#;
}
