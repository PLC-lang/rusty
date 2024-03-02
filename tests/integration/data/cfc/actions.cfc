<?xml version="1.0" encoding="UTF-8"?>
<pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="main" pouType="program">
    <interface>
        <localVars/>
        <addData>
            <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                <textDeclaration>
                    <content>PROGRAM main
VAR
    a,b : DINT;
END_VAR</content>
                </textDeclaration>
            </data>
        </addData>
    </interface>
    <actions>
        <action name="newAction">
            <body>
                <FBD>
                    <outVariable localId="1" height="20" width="80" executionOrderId="0" negated="false" storage="none">
                        <position x="570" y="100"/>
                        <connectionPointIn>
                            <relPosition x="0" y="10"/>
                            <connection refLocalId="2"/>
                        </connectionPointIn>
                        <expression>a</expression>
                    </outVariable>
                    <inVariable localId="2" height="20" width="80" negated="false">
                        <position x="420" y="100"/>
                        <connectionPointOut>
                            <relPosition x="80" y="10"/>
                        </connectionPointOut>
                        <expression>a +  1</expression>
                    </inVariable>
                </FBD>
            </body>
        </action>
        <action name="newAction2">
            <body>
                <FBD>
                    <inVariable localId="1" height="20" width="80" negated="false">
                        <position x="240" y="120"/>
                        <connectionPointOut>
                            <relPosition x="80" y="10"/>
                        </connectionPointOut>
                        <expression>b +  2</expression>
                    </inVariable>
                    <outVariable localId="2" height="20" width="80" executionOrderId="0" negated="false" storage="none">
                        <position x="390" y="120"/>
                        <connectionPointIn>
                            <relPosition x="0" y="10"/>
                            <connection refLocalId="1"/>
                        </connectionPointIn>
                        <expression>b</expression>
                    </outVariable>
                </FBD>
            </body>
        </action>
    </actions>
    <body>
        <FBD>
            <outVariable localId="3" height="20" width="80" executionOrderId="0" negated="false" storage="none">
                    <position x="570" y="100"/>
                    <connectionPointIn>
                            <relPosition x="0" y="10"/>
                            <connection refLocalId="4"/>
                    </connectionPointIn>
                    <expression>a</expression>
            </outVariable>
            <inVariable localId="4" height="20" width="80" negated="false">
                    <position x="420" y="100"/>
                    <connectionPointOut>
                            <relPosition x="80" y="10"/>
                    </connectionPointOut>
                    <expression>0</expression>
            </inVariable>
            <block localId="1" width="100" height="40" typeName="newAction" executionOrderId="1">
                <position x="220" y="170"/>
                <inputVariables/>
                <inOutVariables/>
                <outputVariables/>
            </block>
            <block localId="2" width="110" height="40" typeName="newAction2" executionOrderId="2">
                <position x="220" y="230"/>
                <inputVariables/>
                <inOutVariables/>
                <outputVariables/>
            </block>
        </FBD>
    </body>
</pou>
