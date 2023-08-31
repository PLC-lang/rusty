<?xml version="1.0" encoding="UTF-8"?>
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
