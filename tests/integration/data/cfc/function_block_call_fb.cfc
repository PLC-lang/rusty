<?xml version="1.0" encoding="UTF-8"?>
<pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="myFb" pouType="functionBlock">
    <interface>
        <localVars/>
        <addData>
            <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                <textDeclaration>
                    <content>
FUNCTION_BLOCK myFb
VAR_INPUT
in1 : DINT;
in2 : DINT;
END_VAR

VAR_OUTPUT
out1 : DINT;
out2 : DINT;
out3 : DINT;
END_VAR

VAR

END_VAR
                    </content>
                </textDeclaration>
            </data>
        </addData>
    </interface>
    <body>
        <FBD/>
    </body>
</pou>
