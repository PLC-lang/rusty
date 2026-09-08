<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:Program xmlns:bmx="http://www.bachmann.at/xml/PLC" xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="program_0">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <bmx:TextDeclaration>PROGRAM program_0
VAR
    result: DINT;
END_VAR
</bmx:TextDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="doNothing" globalId="8">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="0"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="450" y="150"/>
                    <ppx:Size x="100" y="40"/>
                    <ppx:InOutVariables/>
                    <ppx:InputVariables>
                        <ppx:InputVariable parameterName="in" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="30"/>
<ppx:Connection refConnectionPointOutId="10"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                    </ppx:InputVariables>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="OUT" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="9">
<ppx:RelPosition x="100" y="30"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="5" globalId="11">
                    <ppx:RelPosition x="300" y="170"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="10">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="result" globalId="12">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="1"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="620" y="170"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="9"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="doNothing2" globalId="13">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="2"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="450" y="240"/>
                    <ppx:Size x="110" y="40"/>
                    <ppx:InOutVariables/>
                    <ppx:InputVariables>
                        <ppx:InputVariable parameterName="in" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="30"/>
<ppx:Connection refConnectionPointOutId="15"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                    </ppx:InputVariables>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="14">
<ppx:RelPosition x="110" y="30"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="5" globalId="16">
                    <ppx:RelPosition x="300" y="260"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="15">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:Program>
