<?xml version="1.0" encoding="UTF-8"?>
<pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="assignment" pouType="functionBlock">
    <interface>
        <localVars/>
        <addData>
            <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                <textDeclaration>
                    <content>
FUNCTION_BLOCK assignment
VAR
    a, b: DINT;
END_VAR
                    </content>
                </textDeclaration>
            </data>
        </addData>
    </interface>
    <body>
        <FBD>
            <inVariable localId="1" height="20" width="80" negated="false">
                <position x="210" y="160"/>
                <connectionPointOut>
                    <relPosition x="80" y="10"/>
                </connectionPointOut>
                <expression>a</expression>
            </inVariable>
            <outVariable localId="2" height="20" width="80" executionOrderId="0" negated="false" storage="none">
                <position x="450" y="160"/>
                <connectionPointIn>
                    <relPosition x="0" y="10"/>
                    <connection refLocalId="1"/>
                </connectionPointIn>
                <expression>b</expression>
            </outVariable>
        </FBD>
    </body>
</pou>