<?xml version="1.0" encoding="UTF-8"?>
<pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="main" pouType="function">
    <interface>
        <localVars/>
        <addData>
            <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                <textDeclaration>
                    <content>
FUNCTION main : DINT
VAR
	x: BOOL := FALSE;
	a: DINT := 0;
END_VAR
					</content>
                </textDeclaration>
            </data>
        </addData>
    </interface>
    <body>
        <FBD>
            <inVariable localId="1" height="20" width="80" negated="false">
                <position x="170" y="160"/>
                <connectionPointOut>
                    <relPosition x="80" y="10"/>
                </connectionPointOut>
                <expression>x</expression>
            </inVariable>
            <label localId="2" height="20" width="80" label="lbl" executionOrderId="5">
                <position x="570" y="50"/>
            </label>
            <label localId="20" height="20" width="80" label="lbl2" executionOrderId="50">
                <position x="570" y="50"/>
            </label>
            <jump localId="3" height="20" width="80" label="lbl" executionOrderId="1">
                <position x="330" y="160"/>
                <connectionPointIn>
                    <relPosition x="0" y="10"/>
                    <connection refLocalId="1"/>
                </connectionPointIn>
                <addData>
                    <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                        <negated value="false"/>
                    </data>
                </addData>
            </jump>
            <outVariable localId="4" height="20" width="80" executionOrderId="2" negated="false" storage="none">
                <position x="320" y="190"/>
                <connectionPointIn>
                    <relPosition x="0" y="10"/>
                    <connection refLocalId="5"/>
                </connectionPointIn>
                <expression>a</expression>
            </outVariable>
            <inVariable localId="5" height="20" width="80" negated="false">
                <position x="170" y="190"/>
                <connectionPointOut>
                    <relPosition x="80" y="10"/>
                </connectionPointOut>
                <expression>2</expression>
            </inVariable>
            <outVariable localId="6" height="20" width="80" executionOrderId="6" negated="false" storage="none">
                <position x="790" y="110"/>
                <connectionPointIn>
                    <relPosition x="0" y="10"/>
                    <connection refLocalId="7"/>
                </connectionPointIn>
                <expression>a</expression>
            </outVariable>
            <inVariable localId="7" height="20" width="80" negated="false">
                <position x="640" y="110"/>
                <connectionPointOut>
                    <relPosition x="80" y="10"/>
                </connectionPointOut>
                <expression>a + 3</expression>
            </inVariable>
            <inVariable localId="8" height="20" width="80" negated="false">
                <position x="170" y="70"/>
                <connectionPointOut>
                    <relPosition x="80" y="10"/>
                </connectionPointOut>
                <expression>TRUE</expression>
            </inVariable>
            <outVariable localId="9" height="20" width="80" executionOrderId="0" negated="false" storage="none">
                <position x="320" y="70"/>
                <connectionPointIn>
                    <relPosition x="0" y="10"/>
                    <connection refLocalId="8"/>
                </connectionPointIn>
                <expression>x</expression>
            </outVariable>
            <comment localId="10" height="20" width="296">
                <position x="140" y="30"/>
                <content>
                    <pre>Set the jump to false, a should be 5 at the end</pre>
                </content>
            </comment>
            <comment localId="11" height="20" width="290">
                <position x="150" y="130"/>
                <content>
                    <pre>This should not be skipped</pre>
                </content>
            </comment>
            <outVariable localId="12" height="20" width="80" executionOrderId="7" negated="false" storage="none">
                <position x="830" y="250"/>
                <connectionPointIn>
                    <relPosition x="0" y="10"/>
                    <connection refLocalId="13"/>
                </connectionPointIn>
                <expression>main</expression>
            </outVariable>
            <inVariable localId="13" height="20" width="80" negated="false">
                <position x="680" y="250"/>
                <connectionPointOut>
                    <relPosition x="80" y="10"/>
                </connectionPointOut>
                <expression>a</expression>
            </inVariable>
        </FBD>
    </body>
</pou>
