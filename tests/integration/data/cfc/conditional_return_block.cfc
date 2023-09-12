<?xml version="1.0" encoding="UTF-8"?>
<pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="conditional_return" pouType="functionBlock">
    <interface>
        <localVars />
        <addData>
            <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                <textDeclaration>
                    <content>
                        FUNCTION_BLOCK conditional_return
                        VAR_INPUT
                        a,b : DINT;
                        res : DINT;
                        END_VAR </content>
                </textDeclaration>
            </data>
        </addData>
    </interface>
    <body>
        <FBD>
            <inVariable localId="1" height="20" width="80" negated="false">
                <position x="120" y="150" />
                <connectionPointOut>
                    <relPosition x="80" y="10" />
                </connectionPointOut>
                <expression>a</expression>
            </inVariable>
            <inVariable localId="2" height="20" width="80" negated="false">
                <position x="120" y="170" />
                <connectionPointOut>
                    <relPosition x="80" y="10" />
                </connectionPointOut>
                <expression>b</expression>
            </inVariable>
            <return localId="4" height="20" width="76" executionOrderId="1">
                <position x="340" y="150" />
                <connectionPointIn>
                    <relPosition x="0" y="10" />
                    <connection refLocalId="8" formalParameter="MyGT" />
                </connectionPointIn>
                <addData>
                    <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                        <negated value="false" />
                    </data>
                </addData>
            </return>
            <inVariable localId="5" height="20" width="80" negated="false">
                <position x="120" y="220" />
                <connectionPointOut>
                    <relPosition x="80" y="10" />
                </connectionPointOut>
                <expression>10</expression>
            </inVariable>
            <outVariable localId="6" height="20" width="80" executionOrderId="2" negated="false"
                storage="none">
                <position x="230" y="220" />
                <connectionPointIn>
                    <relPosition x="0" y="10" />
                    <connection refLocalId="5" />
                </connectionPointIn>
                <expression>res</expression>
            </outVariable>
            <block localId="8" width="74" height="60" typeName="MyGT" executionOrderId="0">
                <position x="230" y="130" />
                <inputVariables>
                    <variable formalParameter="a" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="30" />
                            <connection refLocalId="1" />
                        </connectionPointIn>
                    </variable>
                    <variable formalParameter="b" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="50" />
                            <connection refLocalId="2" />
                        </connectionPointIn>
                    </variable>
                </inputVariables>
                <inOutVariables />
                <outputVariables>
                    <variable formalParameter="MyGT" negated="false">
                        <connectionPointOut>
                            <relPosition x="74" y="30" />
                        </connectionPointOut>
                    </variable>
                </outputVariables>
            </block>
        </FBD>
    </body>
</pou>