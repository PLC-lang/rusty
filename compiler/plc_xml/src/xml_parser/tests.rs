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

        let statements = &parse(content).0.implementations[0].statements;

        assert_eq!(statements.len(), 2);
        assert_debug_snapshot!(statements[0]);
    }

    #[test]
    fn model_is_sorted_by_execution_order() {
        assert_debug_snapshot!(xml_parser::visit(content::EXEC_SORTING).unwrap());
    }

    #[test]
    fn connection_variable_source_to_multiple_sinks_parses() {
        assert_debug_snapshot!(parse(content::VAR_SOURCE_TO_MULTI_SINK).0.implementations[0].statements);
    }

    #[test]
    #[ignore = "block-to-block connections not yet implemented"]
    fn connection_block_source_to_multiple_sinks_parses() {
        assert_debug_snapshot!(parse(content::BLOCK_SOURCE_TO_MULTI_SINK).0.implementations[0].statements);
    }

    #[test]
    fn direct_connection_of_sink_to_other_source_generates_correct_model() {
        assert_debug_snapshot!(xml_parser::visit(content::SINK_TO_SOURCE).unwrap());
    }

    #[test]
    fn direct_connection_of_sink_to_other_source_ast_parses() {
        assert_debug_snapshot!(parse(content::SINK_TO_SOURCE).0.implementations[0].statements);
    }

    #[test]
    fn sink_source_data_recursion_does_not_overflow_the_stack() {
        let Err(diagnostics) = xml_parser::visit(content::SINK_SOURCE_LOOP) else {
            panic!("Expected test to report data recursion!")
        };
        assert_debug_snapshot!(diagnostics);
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

    pub(super) const EXEC_SORTING: &str = r#"
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

    pub(super) const VAR_SOURCE_TO_MULTI_SINK: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
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
                <outVariable localId="4" height="20" width="124" executionOrderId="2" negated="false" storage="none">
                    <position x="1100" y="180"/>
                    <connectionPointIn>
                        <relPosition x="0" y="10"/>
                        <connection refLocalId="9" formalParameter="myAdd">
                            <position x="1100" y="190"/>
                            <position x="1070" y="190"/>
                            <position x="1070" y="220"/>
                            <position x="1050" y="220"/>
                        </connection>
                    </connectionPointIn>
                    <expression>myConnection</expression>
                </outVariable>
                <inVariable localId="7" height="20" width="80" negated="false">
                    <position x="830" y="200"/>
                    <connectionPointOut>
                        <relPosition x="80" y="10"/>
                    </connectionPointOut>
                    <expression>y</expression>
                </inVariable>
                <outVariable localId="8" height="20" width="80" executionOrderId="0" negated="false" storage="none">
                    <position x="850" y="340"/>
                    <connectionPointIn>
                        <relPosition x="0" y="10"/>
                        <connection refLocalId="3"/>
                    </connectionPointIn>
                    <expression>y</expression>
                </outVariable>
                <block localId="9" width="80" height="60" typeName="myAdd" executionOrderId="1">
                    <position x="970" y="190"/>
                    <inputVariables>
                        <variable formalParameter="a" negated="false">
                            <connectionPointIn>
                                <relPosition x="0" y="30"/>
                                <connection refLocalId="7">
                                    <position x="970" y="220"/>
                                    <position x="940" y="220"/>
                                    <position x="940" y="210"/>
                                    <position x="910" y="210"/>
                                </connection>
                            </connectionPointIn>
                        </variable>
                        <variable formalParameter="b" negated="false">
                            <connectionPointIn>
                                <relPosition x="0" y="50"/>
                                <connection refLocalId="3">
                                    <position x="970" y="240"/>
                                    <position x="860" y="240"/>
                                    <position x="860" y="300"/>
                                    <position x="810" y="300"/>
                                    <position x="810" y="350"/>
                                    <position x="774" y="350"/>
                                </connection>
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
            </FBD>
        </body>
    </pou>
    "#;

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

    pub(super) const SINK_TO_SOURCE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
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
                </content>
                    </textDeclaration>
                </data>
            </addData>
        </interface>
        <body>
            <FBD>
                <connector name="s1" localId="1" height="20" width="54">
                    <position x="330" y="190"/>
                    <connectionPointIn>
                        <relPosition x="0" y="10"/>
                        <connection refLocalId="16"/>
                    </connectionPointIn>
                </connector>
                <continuation name="s1" localId="3" height="20" width="64">
                    <position x="420" y="190"/>
                    <connectionPointOut>
                        <relPosition x="64" y="10"/>
                    </connectionPointOut>
                </continuation>
                <outVariable localId="4" height="20" width="124" executionOrderId="3" negated="false" storage="none">
                    <position x="720" y="190"/>
                    <connectionPointIn>
                        <relPosition x="0" y="10"/>
                        <connection refLocalId="20"/>
                    </connectionPointIn>
                    <expression>myConnection</expression>
                </outVariable>
                <inVariable localId="16" height="20" width="80" negated="false">
                    <position x="190" y="190"/>
                    <connectionPointOut>
                        <relPosition x="80" y="10"/>
                    </connectionPointOut>
                    <expression>x</expression>
                </inVariable>
                <connector name="s2" localId="21" height="20" width="54">
                    <position x="520" y="190"/>
                    <connectionPointIn>
                        <relPosition x="0" y="10"/>
                        <connection refLocalId="3"/>
                    </connectionPointIn>
                </connector>
                <continuation name="s2" localId="20" height="20" width="64">
                    <position x="600" y="190"/>
                    <connectionPointOut>
                        <relPosition x="64" y="10"/>
                    </connectionPointOut>
                </continuation>
            </FBD>
        </body>
    </pou>
    "#;

    pub(super) const SINK_SOURCE_LOOP: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
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
                </content>
                    </textDeclaration>
                </data>
            </addData>
        </interface>
        <body>
            <FBD>
                <connector name="s1" localId="22" height="20" width="54">
                    <position x="550" y="160"/>
                    <connectionPointIn>
                        <relPosition x="0" y="10"/>
                        <connection refLocalId="23"/>
                    </connectionPointIn>
                </connector>
                <continuation name="s1" localId="24" height="20" width="64">
                    <position x="630" y="160"/>
                    <connectionPointOut>
                        <relPosition x="64" y="10"/>
                    </connectionPointOut>
                </continuation>
                <connector name="s2" localId="25" height="20" width="54">
                    <position x="740" y="120"/>
                    <connectionPointIn>
                        <relPosition x="0" y="10"/>
                        <connection refLocalId="24">
                            <position x="740" y="130"/>
                            <position x="710" y="130"/>
                            <position x="710" y="170"/>
                            <position x="694" y="170"/>
                        </connection>
                    </connectionPointIn>
                </connector>
                <continuation name="s2" localId="26" height="20" width="64">
                    <position x="750" y="70"/>
                    <connectionPointOut>
                        <relPosition x="64" y="10"/>
                    </connectionPointOut>
                </continuation>
                <connector name="s3" localId="27" height="20" width="54">
                    <position x="850" y="70"/>
                    <connectionPointIn>
                        <relPosition x="0" y="10"/>
                        <connection refLocalId="26"/>
                    </connectionPointIn>
                </connector>
                <continuation name="s3" localId="23" height="20" width="64">
                    <position x="450" y="160"/>
                    <connectionPointOut>
                        <relPosition x="64" y="10"/>
                    </connectionPointOut>
                </continuation>
            </FBD>
        </body>
    </pou>    
    "#;
}
