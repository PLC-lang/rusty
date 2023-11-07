<?xml version="1.0" encoding="UTF-8"?>
<pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="main" pouType="program">
    <interface>
        <localVars/>
        <addData>
            <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                <textDeclaration>
                    <content>
PROGRAM main
VAR
fb0 : myFb;
END_VAR
					</content>
                </textDeclaration>
            </data>
        </addData>
    </interface>
    <body>
        <FBD>
            <block localId="4" width="137" height="80" typeName="myFb" instanceName="fb0" executionOrderId="0">
                <position x="200" y="110"/>
                <inputVariables>
                    <variable formalParameter="in1" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="30"/>
                        </connectionPointIn>
                    </variable>
                    <variable formalParameter="in2" negated="false">
                        <connectionPointIn>
                            <relPosition x="0" y="50"/>
                        </connectionPointIn>
                    </variable>
                </inputVariables>
                <inOutVariables/>
                <outputVariables>
                    <variable formalParameter="out1" negated="false">
                        <connectionPointOut>
                            <relPosition x="137" y="30"/>
                        </connectionPointOut>
                    </variable>
                    <variable formalParameter="out2" negated="false">
                        <connectionPointOut>
                            <relPosition x="137" y="50"/>
                        </connectionPointOut>
                    </variable>
                    <variable formalParameter="out3" negated="false">
                        <connectionPointOut>
                            <relPosition x="137" y="70"/>
                        </connectionPointOut>
                    </variable>
                </outputVariables>
            </block>
        </FBD>
    </body>
</pou>
