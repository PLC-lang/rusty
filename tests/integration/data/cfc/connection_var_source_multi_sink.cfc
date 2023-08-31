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