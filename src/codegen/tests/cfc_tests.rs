use insta::assert_snapshot;

use crate::test_utils::tests::codegen;

#[test]
#[ignore = "Currently not working because when adding `plc_xml` as a dependency Cargo will tell me i'm dumb for adding a cyclic dependency"]
fn early_return() {
    let result = codegen(content::EARLY_RETURN);
    assert_snapshot!(result)
}

mod content {
    pub(super) const EARLY_RETURN: &str = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="early_return" pouType="functionBlock">
        <interface>
            <localVars/>
            <addData>
                <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                    <textDeclaration>
                        <content>
                            FUNCTION_BLOCK early_return
                            VAR_INPUT
                                val : DINT;
                            END_VAR
                        </content>
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
