<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:Program xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="mainProgram">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <textDeclaration>
                <content>PROGRAM mainProgram
VAR
	input: DINT;
	results: ARRAY[1..5] OF DINT;
END_VAR</content>
            </textDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="function_0" globalId="5">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="0"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="140" y="100"/>
                    <ppx:Size x="130" y="40"/>
                    <ppx:InOutVariables>
                        <ppx:InOutVariable parameterName="inout" negated="false">
                            <ppx:ConnectionPointIn>
                                <ppx:RelPosition x="0" y="30"/>
                                <ppx:Connection refConnectionPointOutId="6"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InOutVariable>
                    </ppx:InOutVariables>
                    <ppx:InputVariables/>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="function_0" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="7">
                                <ppx:RelPosition x="130" y="30"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="results[1]" globalId="8">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="1"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="310" y="120"/>
                    <ppx:Size x="100" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="7"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="input" globalId="9">
                    <ppx:RelPosition x="20" y="120"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="6">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:Program>
