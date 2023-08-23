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
    fn simple_return() {
        let content = r#"
<?xml version="1.0" encoding="UTF-8"?>
<pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="increment_until" pouType="function">
    <interface>
        <localVars/>
        <addData>
            <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                <textDeclaration>
                    <content>FUNCTION increment_until : INT
VAR_INPUT
   value	: DINT;
END_VAR

VAR
	i : DINT := 0;
END_VAR</content>
                </textDeclaration>
            </data>
        </addData>
    </interface>
    <body>
        <FBD>
            <return localId="1" height="20" width="76" executionOrderId="2">
                <position x="510" y="140"/>
                <connectionPointIn>
                    <relPosition x="0" y="10"/>
                    <connection refLocalId="2"/>
                </connectionPointIn>
                <addData>
                    <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                        <negated value="false"/>
                    </data>
                </addData>
            </return>
            <inVariable localId="2" height="20" width="94" negated="false">
                <position x="260" y="140"/>
                <connectionPointOut>
                    <relPosition x="94" y="10"/>
                </connectionPointOut>
                <expression>i = value</expression>
            </inVariable>
            <inVariable localId="3" height="20" width="80" negated="false">
                <position x="260" y="70"/>
                <connectionPointOut>
                    <relPosition x="80" y="10"/>
                </connectionPointOut>
                <expression>i</expression>
            </inVariable>
            <inVariable localId="4" height="20" width="80" negated="false">
                <position x="260" y="90"/>
                <connectionPointOut>
                    <relPosition x="80" y="10"/>
                </connectionPointOut>
                <expression>1</expression>
            </inVariable>
            <block localId="5" width="74" height="60" typeName="ADD" executionOrderId="0">
                <position x="390" y="50"/>
                <inputVariables>
                    <variable formalParameter="" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="30"/>
                            <connection refLocalId="3"/>
                        </connectionPointIn>
                    </variable>
                    <variable formalParameter="" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="50"/>
                            <connection refLocalId="4"/>
                        </connectionPointIn>
                    </variable>
                </inputVariables>
                <inOutVariables/>
                <outputVariables>
                    <variable formalParameter="" negated="false">
                        <connectionPointOut>
                            <relPosition x="74" y="30"/>
                        </connectionPointOut>
                    </variable>
                </outputVariables>
            </block>
            <outVariable localId="6" height="20" width="80" executionOrderId="1" negated="false" storage="none">
                <position x="510" y="70"/>
                <connectionPointIn>
                    <relPosition x="0" y="10"/>
                    <connection refLocalId="5"/>
                </connectionPointIn>
                <expression>i</expression>
            </outVariable>
        </FBD>
    </body>
</pou>
        "#;

        assert_debug_snapshot!(parse(&content));
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
