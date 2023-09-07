use ast::{
    ast::{flatten_expression_list, AstStatement, CompilationUnit, LinkageType},
    provider::IdProvider,
};
use insta::assert_debug_snapshot;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::SourceCode;

use crate::{
    serializer::{
        with_header, XBody, XConnection, XConnectionPointIn, XExpression, XFbd, XInVariable, XOutVariable,
        XPou, XRelPosition,
    },
    xml_parser,
};

fn parse(content: &str) -> (CompilationUnit, Vec<Diagnostic>) {
    let source_code = SourceCode::new(content, "test.cfc");
    xml_parser::parse(&source_code, LinkageType::Internal, IdProvider::default())
}

#[test]
fn variable_assignment() {
    let pou = xml_parser::visit(ASSIGNMENT_A_B).unwrap();
    assert_debug_snapshot!(pou);
}

#[test]
fn model_is_sorted_by_execution_order() {
    let src = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="thistimereallyeasy" pouType="program">
            <interface>
                <localVars/>
                <addData>
                    <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                        <textDeclaration>
                            <content>
        PROGRAM thistimereallyeasy
        VAR
            a, b, c, d : DINT;
        END_VAR
                            </content>
                        </textDeclaration>
                    </data>
                </addData>
            </interface>
            <body>
                <FBD>
                    <inVariable localId="1" height="20" width="80" negated="false">
                        <position x="410" y="130"/>
                        <connectionPointOut>
                            <relPosition x="80" y="10"/>
                        </connectionPointOut>
                        <expression>a</expression>
                    </inVariable>
                    <outVariable localId="2" height="20" width="80" executionOrderId="2" negated="false" storage="none">
                        <position x="550" y="70"/>
                        <connectionPointIn>
                            <relPosition x="0" y="10"/>
                            <connection refLocalId="1"/>
                        </connectionPointIn>
                        <expression>b</expression>
                    </outVariable>
                    <outVariable localId="3" height="20" width="80" executionOrderId="0" negated="false" storage="none">
                        <position x="550" y="130"/>
                        <connectionPointIn>
                            <relPosition x="0" y="10"/>
                            <connection refLocalId="1"/>
                        </connectionPointIn>
                        <expression>c</expression>
                    </outVariable>
                    <outVariable localId="4" height="20" width="80" executionOrderId="1" negated="false" storage="none">
                        <position x="550" y="190"/>
                        <connectionPointIn>
                            <relPosition x="0" y="10"/>
                            <connection refLocalId="1"/>
                        </connectionPointIn>
                        <expression>d</expression>
                    </outVariable>
                </FBD>
            </body>
        </pou>
        "#;

    assert_debug_snapshot!(xml_parser::visit(src).unwrap());
}

#[test]
fn function_returns() {
    let content = with_header(
        &XPou::init(
            "FuncyReturn",
            "function",
            "FUNCTION FuncyReturn : DINT
                        VAR_INPUT
                            a : DINT;
                        END_VAR",
        )
        .with_body(
            XBody::new().with_fbd(
                XFbd::new()
                    .with_in_variable(
                        XInVariable::init("1", false).with_expression(XExpression::new().with_data("a")),
                    )
                    .with_out_variable(
                        XOutVariable::init("2", false)
                            .with_attribute("executionOrderId", "0")
                            .with_expression(XExpression::new().with_data("FuncyReturn"))
                            .with_connection_point_in(
                                XConnectionPointIn::new()
                                    .with_rel_position(XRelPosition::init().close())
                                    .with_connection(
                                        XConnection::new().with_attribute("refLocalId", "1").close(),
                                    ),
                            ),
                    ),
            ),
        )
        .serialize(),
    );

    assert_debug_snapshot!(xml_parser::visit(&content).unwrap());
}

#[test]
fn ast_generates_locations() {
    let source_code = SourceCode::new(CALL_BLOCK, "<internal>.cfc");
    let (units, diagnostics) = xml_parser::parse(&source_code, LinkageType::Internal, IdProvider::default());
    let impl1 = &units.implementations[0];
    //Deconstruct assignment and get locations
    let AstStatement::Assignment { left, right, .. } = &impl1.statements[0] else {
        panic!("Not an assignment");
    };
    assert_debug_snapshot!(left.get_location());
    assert_debug_snapshot!(right.get_location());
    //Deconstruct call statement and get locations
    let AstStatement::CallStatement { operator, parameters, location, .. } = &impl1.statements[1] else {
        panic!("Not a call statement");
    };
    assert_debug_snapshot!(location);
    assert_debug_snapshot!(operator.get_location());
    let parameters = parameters.as_ref().as_ref().unwrap();
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
    let source_code = SourceCode::new(ASSIGNMENT_TO_UNRESOLVED_REFERENCE, "<internal>.cfc");
    let (units, diagnostics) = xml_parser::parse(&source_code, LinkageType::Internal, IdProvider::default());
    let impl1 = &units.implementations[0];
    assert_debug_snapshot!(impl1);
    assert!(diagnostics.is_empty());
    //Run resolve and validate
    todo!("Validation in tests not yet done")
}

const ASSIGNMENT_A_B: &str = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="thistimereallyeasy" pouType="program">
            <interface>
                <localVars/>
                <addData>
                    <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                        <textDeclaration>
                            <content>
        PROGRAM thistimereallyeasy
        VAR
            a, b : DINT;
        END_VAR
                            </content>
                        </textDeclaration>
                    </data>
                </addData>
            </interface>
            <body>
                <FBD>
                    <outVariable localId="2" height="20" width="80" executionOrderId="0" negated="false" storage="none">
                        <position x="550" y="130"/>
                        <connectionPointIn>
                            <relPosition x="0" y="10"/>
                            <connection refLocalId="1"/>
                        </connectionPointIn>
                        <expression>b</expression>
                    </outVariable>
                    <inVariable localId="1" height="20" width="80" negated="false">
                    <position x="410" y="130"/>
                    <connectionPointOut>
                        <relPosition x="80" y="10"/>
                    </connectionPointOut>
                    <expression>a</expression>
                </inVariable>
                </FBD>
            </body>
        </pou>
    "#;

const CALL_BLOCK: &str = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="program_0" pouType="program">
            <interface>
                <localVars/>
                <addData>
                    <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                        <textDeclaration>
                            <content>
        PROGRAM program_0
        VAR
        	x : DINT;
        	a : DINT;
        END_VAR
        					</content>
                        </textDeclaration>
                    </data>
                </addData>
            </interface>
            <body>
                <FBD>
                    <inVariable localId="1" height="20" width="80" negated="false">
                        <position x="280" y="80"/>
                        <connectionPointOut>
                            <relPosition x="80" y="10"/>
                        </connectionPointOut>
                        <expression>x</expression>
                    </inVariable>
                    <outVariable localId="2" height="20" width="80" executionOrderId="0" negated="false" storage="none">
                        <position x="520" y="170"/>
                        <connectionPointIn>
                            <relPosition x="0" y="10"/>
                            <connection refLocalId="1">
                                <position x="520" y="180"/>
                                <position x="430" y="180"/>
                                <position x="430" y="90"/>
                                <position x="360" y="90"/>
                            </connection>
                        </connectionPointIn>
                        <expression>a</expression>
                    </outVariable>
                    <block localId="3" width="60" height="60" typeName="ADD" executionOrderId="1">
                        <position x="190" y="160"/>
                        <inputVariables>
                            <variable formalParameter="" negated="false">
                                <connectionPointIn>
                                    <relPosition x="0" y="30"/>
                                    <connection refLocalId="4"/>
                                </connectionPointIn>
                            </variable>
                            <variable formalParameter="" negated="false">
                                <connectionPointIn>
                                    <relPosition x="0" y="50"/>
                                    <connection refLocalId="5"/>
                                </connectionPointIn>
                            </variable>
                        </inputVariables>
                        <inOutVariables/>
                        <outputVariables>
                            <variable formalParameter="" negated="false">
                                <connectionPointOut>
                                    <relPosition x="60" y="30"/>
                                </connectionPointOut>
                            </variable>
                        </outputVariables>
                    </block>
                    <inVariable localId="4" height="20" width="80" negated="false">
                        <position x="40" y="180"/>
                        <connectionPointOut>
                            <relPosition x="80" y="10"/>
                        </connectionPointOut>
                        <expression>a</expression>
                    </inVariable>
                    <inVariable localId="5" height="20" width="80" negated="false">
                        <position x="40" y="200"/>
                        <connectionPointOut>
                            <relPosition x="80" y="10"/>
                        </connectionPointOut>
                        <expression>1</expression>
                    </inVariable>
                </FBD>
            </body>
        </pou>
        
    "#;

const ASSIGNMENT_TO_UNRESOLVED_REFERENCE: &str = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="program_0" pouType="program">
            <interface>
                <localVars/>
                <addData>
                    <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                        <textDeclaration>
                            <content>
        PROGRAM program_0
        VAR
        	x : DINT;
        END_VAR
        					</content>
                        </textDeclaration>
                    </data>
                </addData>
            </interface>
            <body>
                <FBD>
                    <inVariable localId="1" height="20" width="80" negated="false">
                        <position x="280" y="80"/>
                        <connectionPointOut>
                            <relPosition x="80" y="10"/>
                        </connectionPointOut>
                        <expression>x</expression>
                    </inVariable>
                    <outVariable localId="2" height="20" width="80" executionOrderId="0" negated="false" storage="none">
                        <position x="520" y="170"/>
                        <connectionPointIn>
                            <relPosition x="0" y="10"/>
                            <connection refLocalId="1">
                                <position x="520" y="180"/>
                                <position x="430" y="180"/>
                                <position x="430" y="90"/>
                                <position x="360" y="90"/>
                            </connection>
                        </connectionPointIn>
                        <expression>a</expression>
                    </outVariable>
                </FBD>
            </body>
        </pou>
        
    "#;
