<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:Program xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="function_inout_unwired">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <textDeclaration>
                <content>PROGRAM function_inout_unwired
VAR
    result : DINT;
END_VAR</content>
            </textDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="addInto" globalId="1">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="0"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="600" y="350"/>
                    <ppx:Size x="130" y="70"/>
                    <ppx:InputVariables>
                        <ppx:InputVariable parameterName="delta" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="30"/>
<ppx:Connection refConnectionPointOutId="2"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                    </ppx:InputVariables>
                    <ppx:InOutVariables>
                        <ppx:InOutVariable parameterName="acc" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="50"/>
                            </ppx:ConnectionPointIn>
                            <ppx:ConnectionPointOut connectionPointOutId="5">
<ppx:RelPosition x="130" y="50"/>
                            </ppx:ConnectionPointOut>
                        </ppx:InOutVariable>
                    </ppx:InOutVariables>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="addInto" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="4">
<ppx:RelPosition x="130" y="30"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="5" globalId="6">
                    <ppx:RelPosition x="480" y="370"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="2">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="result" globalId="8">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="1"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="770" y="370"/>
                    <ppx:Size x="90" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="4"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:Program>
