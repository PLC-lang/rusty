#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use plc::{
        ast::{CompilationUnit, LinkageType},
        diagnostics::Diagnostic,
        lexer::IdProvider,
    };

    use crate::{
        deserializer,
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
        let pou = crate::deserializer::visit(ASSIGNMENT_A_B).unwrap();
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

        assert_debug_snapshot!(deserializer::visit(src).unwrap());
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

        assert_debug_snapshot!(deserializer::visit(&content).unwrap());
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
