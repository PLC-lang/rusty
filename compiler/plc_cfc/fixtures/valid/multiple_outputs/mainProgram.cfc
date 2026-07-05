<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:Program xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="mainProgram">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <textDeclaration>
                <content>PROGRAM mainProgram
VAR
	myInstance: myFunctionBlock;
	localA: DINT;
	localB: DINT;
END_VAR
                </content>
            </textDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="myFunctionBlock" instanceName="myInstance" globalId="1">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="0"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="180" y="140"/>
                    <ppx:Size x="130" y="80"/>
                    <ppx:InOutVariables/>
                    <ppx:InputVariables/>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="a" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="2">
                                <ppx:RelPosition x="130" y="30"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                        <ppx:OutputVariable parameterName="b" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="3">
                                <ppx:RelPosition x="130" y="50"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                        <ppx:OutputVariable parameterName="c" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="4">
                                <ppx:RelPosition x="130" y="70"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="localA" globalId="5">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="1"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="350" y="160"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="2"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="localB" globalId="6">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="2"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="350" y="200"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="4"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="myFunction" globalId="7">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="3"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="180" y="240"/>
                    <ppx:Size x="100" y="80"/>
                    <ppx:InOutVariables/>
                    <ppx:InputVariables/>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="myFunction" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="8">
                                <ppx:RelPosition x="100" y="30"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                        <ppx:OutputVariable parameterName="a" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="9">
                                <ppx:RelPosition x="100" y="50"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                        <ppx:OutputVariable parameterName="b" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="10">
                                <ppx:RelPosition x="100" y="70"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="localA" globalId="11">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="4"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="340" y="280"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="9"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:Program>
