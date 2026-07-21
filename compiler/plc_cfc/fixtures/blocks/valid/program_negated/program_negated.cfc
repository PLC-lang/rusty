<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:Program xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="program_negated">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <textDeclaration>
                <content>PROGRAM program_negated
VAR
    localIn : DINT;
    localInOut : DINT;
    localOut : DINT;
END_VAR</content>
            </textDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="program_0" globalId="7">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="0"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="570" y="130"/>
                    <ppx:Size x="100" y="60"/>
                    <ppx:InOutVariables>
                        <ppx:InOutVariable parameterName="inout" negated="true">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="50"/>
<ppx:Connection refConnectionPointOutId="10"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InOutVariable>
                    </ppx:InOutVariables>
                    <ppx:InputVariables>
                        <ppx:InputVariable parameterName="in" negated="true">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="30"/>
<ppx:Connection refConnectionPointOutId="9"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                    </ppx:InputVariables>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="out" negated="true">
                            <ppx:ConnectionPointOut connectionPointOutId="8">
<ppx:RelPosition x="100" y="30"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="localIn" globalId="11">
                    <ppx:RelPosition x="440" y="150"/>
                    <ppx:Size x="90" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="9">
                        <ppx:RelPosition x="90" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="localInOut" globalId="12">
                    <ppx:RelPosition x="420" y="170"/>
                    <ppx:Size x="110" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="10">
                        <ppx:RelPosition x="110" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="localOut" globalId="13">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="1"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="710" y="150"/>
                    <ppx:Size x="90" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="8"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:Program>
