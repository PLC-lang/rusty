#[cfg(test)]
mod tests {
    use ast::{
        ast::{CompilationUnit, LinkageType},
        provider::IdProvider,
    };
    use insta::assert_debug_snapshot;
    use plc_diagnostics::diagnostics::Diagnostic;

    use crate::{
        serializer::{
            with_header, XBody, XConnection, XConnectionPointIn, XExpression, XFbd, XInVariable,
            XOutVariable, XPou, XRelPosition,
        },
        xml_parser::{self, tests::ASSIGNMENT_A_B},
    };

    fn parse(content: &str) -> (CompilationUnit, Vec<Diagnostic>) {
        xml_parser::parse(content, "test.cfc", LinkageType::Internal, IdProvider::default())
    }

    #[test]
    fn variable_assignment() {
        let pou = xml_parser::visit(ASSIGNMENT_A_B).unwrap();
        assert_debug_snapshot!(pou);
    }

    #[test]
    fn connection_sink_source() {
        let src = r#"<?xml version="1.0" encoding="UTF-8"?>
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
                        <position x="450" y="330"/>
                        <connectionPointIn>
                            <relPosition x="0" y="10"/>
                            <connection refLocalId="2"/>
                        </connectionPointIn>
                    </connector>
                    <continuation name="s1" localId="3" height="20" width="64">
                        <position x="710" y="340"/>
                        <connectionPointOut>
                            <relPosition x="64" y="10"/>
                        </connectionPointOut>
                    </continuation>
                    <inVariable localId="2" height="20" width="80" negated="false">
                        <position x="340" y="330"/>
                        <connectionPointOut>
                            <relPosition x="80" y="10"/>
                        </connectionPointOut>
                        <expression>x</expression>
                    </inVariable>
                    <outVariable localId="4" height="20" width="124" executionOrderId="0" negated="false" storage="none">
                        <position x="840" y="340"/>
                        <connectionPointIn>
                            <relPosition x="0" y="10"/>
                            <connection refLocalId="3"/>
                        </connectionPointIn>
                        <expression>myConnection</expression>
                    </outVariable>
                    <outVariable localId="5" height="20" width="80" executionOrderId="1" negated="false" storage="none">
                        <position x="840" y="220"/>
                        <connectionPointIn>
                            <relPosition x="0" y="10"/>
                            <connection refLocalId="3"/>
                        </connectionPointIn>
                        <expression>y</expression>
                    </outVariable>
                </FBD>
            </body>
        </pou>
           
        "#;

        assert_debug_snapshot!(xml_parser::visit(src).unwrap());
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
