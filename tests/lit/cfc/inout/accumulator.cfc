<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:Program xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="Accumulator">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <textDeclaration>
                <content>PROGRAM Accumulator
VAR
	value : DINT;
	sum : DINT;
	result : DINT;
END_VAR</content>
            </textDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="value" globalId="1">
                    <ppx:RelPosition x="100" y="60"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="2">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="sum" globalId="3">
                    <ppx:RelPosition x="100" y="120"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="4">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="accumulate" globalId="5">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="1"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="300" y="60"/>
                    <ppx:Size x="120" y="60"/>
                    <ppx:InOutVariables>
                        <ppx:InOutVariable parameterName="sum" negated="false">
                            <ppx:ConnectionPointIn>
                                <ppx:RelPosition x="0" y="40"/>
                                <ppx:Connection refConnectionPointOutId="4"/>
                            </ppx:ConnectionPointIn>
                            <ppx:ConnectionPointOut connectionPointOutId="8">
                                <ppx:RelPosition x="120" y="40"/>
                            </ppx:ConnectionPointOut>
                        </ppx:InOutVariable>
                    </ppx:InOutVariables>
                    <ppx:InputVariables>
                        <ppx:InputVariable parameterName="value" negated="false">
                            <ppx:ConnectionPointIn>
                                <ppx:RelPosition x="0" y="20"/>
                                <ppx:Connection refConnectionPointOutId="2"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                    </ppx:InputVariables>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="accumulate" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="6">
                                <ppx:RelPosition x="120" y="20"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="result" globalId="7">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="2"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="560" y="60"/>
                    <ppx:Size x="100" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="6"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:Program>
