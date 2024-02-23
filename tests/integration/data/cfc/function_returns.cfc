<?xml version="1.0" encoding="UTF-8"?>
<pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="FuncyReturn" pouType="function">
    <interface>
        <localVars/>
        <addData>
            <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                <textDeclaration>
                    <content>
FUNCTION FuncyReturn : DINT
VAR_INPUT
    a : DINT;
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
            <inVariable localId="1" height="20" width="80" negated="false">
                <position x="180" y="110"/>
                <connectionPointOut>
                    <relPosition x="80" y="10"/>
                </connectionPointOut>
                <expression>a * 2</expression>
            </inVariable>
            <outVariable localId="4" height="20" width="117" executionOrderId="0" negated="false" storage="none">
                <position x="580" y="110"/>
                <connectionPointIn>
                    <relPosition x="0" y="10"/>
                    <connection refLocalId="1"/>
                </connectionPointIn>
                <expression>FuncyReturn</expression>
            </outVariable>
        </FBD>
    </body>
</pou>
