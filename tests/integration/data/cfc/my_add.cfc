<?xml version="1.0" encoding="UTF-8"?>
<pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="myAdder" pouType="functionBlock">
    <interface>
        <localVars/>
        <addData>
            <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                <textDeclaration>
                    <content>
FUNCTION_BLOCK myAdder
VAR
	x, y: DINT;
END_VAR

VAR_OUTPUT

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
            <block localId="1" width="80" height="60" typeName="myAdd" executionOrderId="0">
                <position x="630" y="150"/>
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
                    <variable formalParameter="myAdd" negated="false">
                        <connectionPointOut>
                            <relPosition x="80" y="30"/>
                        </connectionPointOut>
                    </variable>
                </outputVariables>
            </block>
            <inVariable localId="2" height="20" width="80" negated="false">
                <position x="490" y="160"/>
                <connectionPointOut>
                    <relPosition x="80" y="10"/>
                </connectionPointOut>
                <expression>x</expression>
            </inVariable>
            <inVariable localId="3" height="20" width="80" negated="false">
                <position x="490" y="200"/>
                <connectionPointOut>
                    <relPosition x="80" y="10"/>
                </connectionPointOut>
                <expression>y + 1</expression>
            </inVariable>
            <outVariable localId="4" height="20" width="80" executionOrderId="1" negated="false" storage="none">
                <position x="780" y="170"/>
                <connectionPointIn>
                    <relPosition x="0" y="10"/>
                    <connection refLocalId="1" formalParameter="myAdd"/>
                </connectionPointIn>
                <expression>x</expression>
            </outVariable>
        </FBD>
    </body>
</pou>
