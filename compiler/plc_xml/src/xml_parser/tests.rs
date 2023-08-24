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
        xml_parser::{self, tests::content},
    };

    fn parse(content: &str) -> (CompilationUnit, Vec<Diagnostic>) {
        xml_parser::parse(content, "test.cfc", LinkageType::Internal, IdProvider::default())
    }

    #[test]
    fn variable_assignment() {
        let pou = xml_parser::visit(content::ASSIGNMENT_A_B).unwrap();
        assert_debug_snapshot!(pou);
    }

    #[test]
    fn conditional_return() {
        let statements = &parse(content::CONDITIONAL_RETURN).0.implementations[0].statements;
        assert_eq!(statements.len(), 2);
        assert_debug_snapshot!(statements[0]);
    }

    #[test]
    fn conditional_return_negated() {
        let content =
            &content::CONDITIONAL_RETURN.replace(r#"<negated value="false"/>"#, r#"<negated value="true"/>"#);

        let statements = &parse(&content).0.implementations[0].statements;

        assert_eq!(statements.len(), 2);
        assert_debug_snapshot!(statements[0]);
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

mod content {
    pub(super) const ASSIGNMENT_A_B: &str = r#"
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

    pub(super) const CONDITIONAL_RETURN: &str = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="conditional_return" pouType="functionBlock">
        <interface>
            <localVars/>
            <addData>
                <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                    <textDeclaration>
                        <content>
    FUNCTION_BLOCK conditional_return
    VAR_INPUT
        val : DINT;
    END_VAR</content>
                    </textDeclaration>
                </data>
            </addData>
        </interface>
        <body>
            <FBD>
                <inVariable localId="1" height="20" width="82" negated="false">
                    <position x="220" y="60"/>
                    <connectionPointOut>
                        <relPosition x="82" y="10"/>
                    </connectionPointOut>
                    <expression>val = 5</expression>
                </inVariable>
                <return localId="2" height="20" width="76" executionOrderId="0">
                    <position x="330" y="60"/>
                    <connectionPointIn>
                        <relPosition x="0" y="10"/>
                        <connection refLocalId="1"/>
                    </connectionPointIn>
                    <addData>
                        <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                            <negated value="false"/>
                        </data>
                    </addData>
                </return>
                <inVariable localId="3" height="20" width="80" negated="false">
                    <position x="220" y="100"/>
                    <connectionPointOut>
                        <relPosition x="80" y="10"/>
                    </connectionPointOut>
                    <expression>10</expression>
                </inVariable>
                <outVariable localId="4" height="20" width="80" executionOrderId="1" negated="false" storage="none">
                    <position x="330" y="100"/>
                    <connectionPointIn>
                        <relPosition x="0" y="10"/>
                        <connection refLocalId="3"/>
                    </connectionPointIn>
                    <expression>val</expression>
                </outVariable>
                <inOutVariable localId="5" height="20" width="80" negatedIn="false" storageIn="none" negatedOut="false">
                    <position x="780" y="60"/>
                    <connectionPointIn>
                        <relPosition x="0" y="10"/>
                    </connectionPointIn>
                    <connectionPointOut>
                        <relPosition x="80" y="10"/>
                    </connectionPointOut>
                    <expression>a</expression>
                </inOutVariable>
            </FBD>
        </body>
    </pou>
    "#;
}
