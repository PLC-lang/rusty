<?xml version="1.0" encoding="UTF-8"?>
<pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="select" pouType="functionBlock">
    <interface>
        <localVars/>
        <addData>
            <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                <textDeclaration>
                    <content>
FUNCTION_BLOCK select
VAR_INPUT
    a, b : DINT;
END_VAR
VAR
    selected: DINT;
END_VAR
                    </content>
                </textDeclaration>
            </data>
        </addData>
    </interface>
    <body>
        <FBD>
            <block localId="1" width="80" height="80" typeName="SEL" executionOrderId="0">
                <position x="540" y="220"/>
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
                    <variable formalParameter="" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="70"/>
                            <connection refLocalId="4"/>
                        </connectionPointIn>
                    </variable>
                </inputVariables>
                <inOutVariables/>
                <outputVariables>
                    <variable formalParameter="" negated="false">
                        <connectionPointOut>
                            <relPosition x="80" y="30"/>
                        </connectionPointOut>
                    </variable>
                </outputVariables>
            </block>
            <inVariable localId="2" height="20" width="96" negated="false">
                <position x="330" y="230"/>
                <connectionPointOut>
                    <relPosition x="96" y="10"/>
                </connectionPointOut>
                <expression>a > b</expression>
            </inVariable>
            <inVariable localId="3" height="20" width="80" negated="false">
                <position x="330" y="260"/>
                <connectionPointOut>
                    <relPosition x="80" y="10"/>
                </connectionPointOut>
                <expression>a</expression>
            </inVariable>
            <inVariable localId="4" height="20" width="80" negated="false">
                <position x="330" y="290"/>
                <connectionPointOut>
                    <relPosition x="80" y="10"/>
                </connectionPointOut>
                <expression>b</expression>
            </inVariable>
            <outVariable localId="5" height="20" width="96" executionOrderId="1" negated="false" storage="none">
                <position x="720" y="240"/>
                <connectionPointIn>
                    <relPosition x="0" y="10"/>
                    <connection refLocalId="1"/>
                </connectionPointIn>
                <expression>selected</expression>
            </outVariable>
        </FBD>
    </body>
</pou>
