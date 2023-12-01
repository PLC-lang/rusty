<?xml version="1.0" encoding="UTF-8"?>
<pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="ridiculous_chaining" pouType="program">
    <interface>
        <localVars/>
        <addData>
            <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                <textDeclaration>
                    <content>
PROGRAM ridiculous_chaining
VAR_INPUT
    x, y : DINT;
END_VAR
VAR
     z : DINT;
END_VAR
                    </content>
                </textDeclaration>
            </data>
        </addData>
    </interface>
    <body>
        <FBD>
            <block localId="1" width="80" height="60" typeName="myAdd" executionOrderId="0">
                <position x="210" y="130"/>
                <inputVariables>
                    <variable formalParameter="a" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="30"/>
                            <connection refLocalId="9"/>
                        </connectionPointIn>
                    </variable>
                    <variable formalParameter="b" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="50"/>
                            <connection refLocalId="10"/>
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
            <block localId="2" width="80" height="60" typeName="myAdd" executionOrderId="2">
                <position x="340" y="170"/>
                <inputVariables>
                    <variable formalParameter="a" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="30"/>
                            <connection refLocalId="1" formalParameter="myAdd"/>
                        </connectionPointIn>
                    </variable>
                    <variable formalParameter="b" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="50"/>
                            <connection refLocalId="1" formalParameter="myAdd"/>
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
            <block localId="3" width="80" height="60" typeName="myAdd" executionOrderId="6">
                <position x="740" y="130"/>
                <inputVariables>
                    <variable formalParameter="a" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="30"/>
                            <connection refLocalId="4" formalParameter="myAdd"/>
                        </connectionPointIn>
                    </variable>
                    <variable formalParameter="b" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="50"/>
                            <connection refLocalId="4" formalParameter="myAdd"/>
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
            <block localId="4" width="80" height="60" typeName="myAdd" executionOrderId="4">
                <position x="610" y="130"/>
                <inputVariables>
                    <variable formalParameter="a" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="30"/>
                            <connection refLocalId="6" formalParameter="myAdd"/>
                        </connectionPointIn>
                    </variable>
                    <variable formalParameter="b" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="50"/>
                            <connection refLocalId="2" formalParameter="myAdd"/>
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
            <block localId="5" width="80" height="60" typeName="myAdd" executionOrderId="5">
                <position x="740" y="50"/>
                <inputVariables>
                    <variable formalParameter="a" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="30"/>
                            <connection refLocalId="6" formalParameter="myAdd"/>
                        </connectionPointIn>
                    </variable>
                    <variable formalParameter="b" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="50"/>
                            <connection refLocalId="4" formalParameter="myAdd"/>
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
            <block localId="6" width="80" height="60" typeName="myAdd" executionOrderId="3">
                <position x="500" y="50"/>
                <inputVariables>
                    <variable formalParameter="a" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="30"/>
                            <connection refLocalId="7" formalParameter="myAdd"/>
                        </connectionPointIn>
                    </variable>
                    <variable formalParameter="b" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="50"/>
                            <connection refLocalId="2" formalParameter="myAdd"/>
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
            <block localId="7" width="80" height="60" typeName="myAdd" executionOrderId="1">
                <position x="340" y="50"/>
                <inputVariables>
                    <variable formalParameter="a" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="30"/>
                            <connection refLocalId="1" formalParameter="myAdd"/>
                        </connectionPointIn>
                    </variable>
                    <variable formalParameter="b" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="50"/>
                            <connection refLocalId="1" formalParameter="myAdd"/>
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
            <block localId="8" width="80" height="60" typeName="myAdd" executionOrderId="7">
                <position x="930" y="40"/>
                <inputVariables>
                    <variable formalParameter="a" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="30"/>
                            <connection refLocalId="5" formalParameter="myAdd"/>
                        </connectionPointIn>
                    </variable>
                    <variable formalParameter="b" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="50"/>
                            <connection refLocalId="3" formalParameter="myAdd"/>
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
            <inVariable localId="9" height="20" width="80" negated="false">
                <position x="80" y="140"/>
                <connectionPointOut>
                    <relPosition x="80" y="10"/>
                </connectionPointOut>
                <expression>x</expression>
            </inVariable>
            <inVariable localId="10" height="20" width="80" negated="false">
                <position x="80" y="180"/>
                <connectionPointOut>
                    <relPosition x="80" y="10"/>
                </connectionPointOut>
                <expression>y</expression>
            </inVariable>
            <outVariable localId="11" height="20" width="80" executionOrderId="8" negated="false" storage="none">
                <position x="1050" y="60"/>
                <connectionPointIn>
                    <relPosition x="0" y="10"/>
                    <connection refLocalId="8" formalParameter="myAdd"/>
                </connectionPointIn>
                <expression>z</expression>
            </outVariable>
            <comment localId="13" height="20" width="80">
                <position x="60" y="200"/>
                <content>
                    <pre>x, y = 2</pre>
                </content>
            </comment>
            <comment localId="14" height="20" width="80">
                <position x="250" y="180"/>
                <content>
                    <pre>4</pre>
                </content>
            </comment>
            <comment localId="15" height="20" width="80">
                <position x="390" y="210"/>
                <content>
                    <pre>8</pre>
                </content>
            </comment>
            <comment localId="16" height="20" width="80">
                <position x="400" y="90"/>
                <content>
                    <pre>8</pre>
                </content>
            </comment>
            <comment localId="17" height="20" width="80">
                <position x="590" y="60"/>
                <content>
                    <pre>16</pre>
                </content>
            </comment>
            <comment localId="18" height="20" width="80">
                <position x="660" y="170"/>
                <content>
                    <pre>32</pre>
                </content>
            </comment>
            <comment localId="19" height="20" width="80">
                <position x="780" y="90"/>
                <content>
                    <pre>48</pre>
                </content>
            </comment>
            <comment localId="20" height="20" width="80">
                <position x="780" y="170"/>
                <content>
                    <pre>64</pre>
                </content>
            </comment>
            <comment localId="21" height="20" width="80">
                <position x="970" y="80"/>
                <content>
                    <pre>112</pre>
                </content>
            </comment>
        </FBD>
    </body>
</pou>
