#[test]
fn simple() {
    insta::assert_debug_snapshot!(crate::deserializer::visit(SIMPLE));

    let _src = r#"
    FUNCTION function_1 : DINT
    VAR_INPUT
        elem: DINT;
    END_VAR
        function_1 := elem + 1;
    END_FUNCTION
    "#;
}

#[test]
fn demo() {
    insta::assert_debug_snapshot!(crate::deserializer::visit(CONTENT));
}

#[test]
fn labels() {
    insta::assert_debug_snapshot!(crate::deserializer::visit(CONTENT_WITH_LABELS));
}

const SIMPLE: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="function_1" pouType="function">
    <interface>
        <localVars/>
        <addData>
            <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                <textDeclaration>
                    <content>
                        FUNCTION function_1 : DINT
                        VAR_INPUT
                            elem: DINT;
                        END_VAR

                        VAR

                        END_VAR
                    </content>
                </textDeclaration>
            </data>
        </addData>
    </interface>
    <body>
        <FBD>
            <block localId="1" width="62" height="60" typeName="ADD" executionOrderId="1">
                <position x="280" y="170"/>
                <inputVariables>
                    <variable formalParameter="" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="30"/>
                            <connection refLocalId="2"/>
                        </connectionPointIn>
                    </variable>
                    <variable formalParameter="" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="50"/>
                            <connection refLocalId="3"/>
                        </connectionPointIn>
                    </variable>
                </inputVariables>
                <inOutVariables/>
                <outputVariables>
                    <variable formalParameter="" negated="false">
                        <connectionPointOut>
                            <relPosition x="62" y="30"/>
                        </connectionPointOut>
                    </variable>
                </outputVariables>
            </block>
            <inVariable localId="2" height="20" width="80" negated="false">
                <position x="120" y="190"/>
                <connectionPointOut>
                    <relPosition x="80" y="10"/>
                </connectionPointOut>
                <expression>elem</expression>
            </inVariable>
            <inVariable localId="3" height="20" width="80" negated="false">
                <position x="120" y="210"/>
                <connectionPointOut>
                    <relPosition x="80" y="10"/>
                </connectionPointOut>
                <expression>1</expression>
            </inVariable>
            <outVariable localId="7" height="20" width="80" executionOrderId="2" negated="false" storage="none">
                <position x="400" y="190"/>
                <connectionPointIn>
                    <relPosition x="0" y="10"/>
                    <connection refLocalId="1"/>
                </connectionPointIn>
                <expression>elem</expression>
            </outVariable>
        </FBD>
    </body>
</pou>
"#;

const CONTENT: &str = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="program_0" pouType="program">
        <!-- <interface>
            <localVars />
            <addData>
                <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                    <textDeclaration>
                        <content>
                            PROGRAM program_0
                            VAR
                            local_a : DINT := 1;
                            local_b : DINT := 2;
                            local_c : DINT := 0;
                            local_add : MyAdd;
                            END_VAR
                        </content>
                    </textDeclaration>
                </data>
            </addData>
        </interface> -->
        <body>
            <FBD>
                <block localId="5" width="82" height="60" typeName="MyAdd" instanceName="local_add"
                    executionOrderId="0">
                    <position x="480" y="150" />
                    <inputVariables>
                        <variable formalParameter="a" negated="false">
                            <connectionPointIn>
                                <relPosition x="0" y="30" />
                                <connection refLocalId="6" />
                            </connectionPointIn>
                        </variable>
                        <variable formalParameter="b" negated="false">
                            <connectionPointIn>
                                <relPosition x="0" y="50" />
                                <connection refLocalId="7" />
                            </connectionPointIn>
                        </variable>
                    </inputVariables>
                    <inOutVariables />
                    <outputVariables>
                        <variable formalParameter="c" negated="false">
                            <connectionPointOut>
                                <relPosition x="82" y="30" />
                            </connectionPointOut>
                        </variable>
                    </outputVariables>
                </block>
                <inVariable localId="6" height="20" width="82" negated="false">
                    <position x="340" y="170" />
                    <connectionPointOut>
                        <relPosition x="82" y="10" />
                    </connectionPointOut>
                    <expression>
                        local_a
                    <!-- comment -->
                    </expression>
                </inVariable>
                <inVariable localId="7" height="20" width="82" negated="false">
                    <position x="340" y="190" />
                    <connectionPointOut>
                        <relPosition x="82" y="10" />
                    </connectionPointOut>
                    <expression>local_b</expression>
                </inVariable>
                <outVariable localId="8" height="20" width="82" executionOrderId="1" negated="false"
                    storage="none">
                    <position x="620" y="170" />
                    <connectionPointIn>
                        <relPosition x="0" y="10" />
                        <connection refLocalId="5" formalParameter="c" />
                    </connectionPointIn>
                    <expression>local_c</expression>
                </outVariable>
            </FBD>
        </body>
    </pou>
"#;

const CONTENT_WITH_LABELS: &str = r#"
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
        local_a : DINT := 1;
        local_b : DINT := 2;
        local_c:  DINT;
        local_bool : BOOL := TRUE;
        local_add : MyAdd;
    END_VAR</content>
                    </textDeclaration>
                </data>
            </addData>
        </interface>
        <body>
            <FBD>
                <block localId="1" width="82" height="60" typeName="MyAdd" instanceName="local_add" executionOrderId="0">
                    <position x="310" y="150"/>
                    <inputVariables>
                        <variable formalParameter="a" negated="false">
                            <connectionPointIn>
                                <relPosition x="0" y="30"/>
                                <connection refLocalId="2"/>
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
                        <variable formalParameter="c" negated="false">
                            <connectionPointOut>
                                <relPosition x="82" y="30"/>
                            </connectionPointOut>
                        </variable>
                    </outputVariables>
                </block>
                <inVariable localId="2" height="20" width="106" negated="false">
                    <position x="170" y="170"/>
                    <connectionPointOut>
                        <relPosition x="106" y="10"/>
                    </connectionPointOut>
                    <expression>local_a + 1</expression>
                </inVariable>
                <inVariable localId="3" height="20" width="82" negated="false">
                    <position x="170" y="190"/>
                    <connectionPointOut>
                        <relPosition x="82" y="10"/>
                    </connectionPointOut>
                    <expression>local_b</expression>
                </inVariable>
                <jump localId="9" height="20" width="87" label="jumpy" executionOrderId="2">
                    <position x="390" y="300"/>
                    <connectionPointIn>
                        <relPosition x="0" y="10"/>
                        <connection refLocalId="12"/>
                    </connectionPointIn>
                    <addData>
                        <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                            <negated value="false"/>
                        </data>
                    </addData>
                </jump>
                <label localId="10" height="20" width="80" label="jumpy" executionOrderId="3">
                    <position x="530" y="300"/>
                </label>
                <return localId="11" height="20" width="76" executionOrderId="4">
                    <position x="790" y="220"/>
                    <connectionPointIn>
                        <relPosition x="0" y="10"/>
                        <connection refLocalId="15"/>
                    </connectionPointIn>
                    <addData>
                        <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                            <negated value="false"/>
                        </data>
                    </addData>
                </return>
                <inVariable localId="12" height="20" width="100" negated="false">
                    <position x="240" y="300"/>
                    <connectionPointOut>
                        <relPosition x="100" y="10"/>
                    </connectionPointOut>
                    <expression>local_bool</expression>
                </inVariable>
                <continuation name="wifi" localId="15" height="20" width="116">
                    <position x="630" y="220"/>
                    <connectionPointOut>
                        <relPosition x="116" y="10"/>
                    </connectionPointOut>
                </continuation>
                <connector name="wifi" localId="16" height="20" width="136">
                    <position x="460" y="220"/>
                    <connectionPointIn>
                        <relPosition x="0" y="10"/>
                        <connection refLocalId="12"/>
                    </connectionPointIn>
                </connector>
                <outVariable localId="17" height="20" width="82" executionOrderId="5" negated="false" storage="none">
                    <position x="440" y="170"/>
                    <connectionPointIn>
                        <relPosition x="0" y="10"/>
                        <connection refLocalId="1" formalParameter="c"/>
                    </connectionPointIn>
                    <expression>local_c</expression>
                </outVariable>
            </FBD>
        </body>
    </pou>
"#;
