<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:FunctionBlock xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="Motor">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <textDeclaration>
                <content>FUNCTION_BLOCK Motor
VAR_INPUT
	start, stop, estop : BOOL;
END_VAR
VAR_OUTPUT
	running : BOOL;
END_VAR
VAR
	latch : SrLatch;
END_VAR</content>
            </textDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="start" globalId="1">
                    <ppx:RelPosition x="60" y="60"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="2">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="stop" globalId="3">
                    <ppx:RelPosition x="60" y="200"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="4">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="estop" globalId="5">
                    <ppx:RelPosition x="60" y="300"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="6">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:CommonObject xsi:type="ppx:Connector" label="estop_wire" globalId="7">
                    <ppx:RelPosition x="200" y="300"/>
                    <ppx:Size x="90" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="6"/>
                    </ppx:ConnectionPointIn>
                </ppx:CommonObject>
                <ppx:CommonObject xsi:type="ppx:Continuation" label="estop_wire" globalId="8">
                    <ppx:RelPosition x="200" y="120"/>
                    <ppx:Size x="90" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="9">
                        <ppx:RelPosition x="90" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:CommonObject>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="StartCmd" globalId="10">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="0"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="400" y="40"/>
                    <ppx:Size x="140" y="60"/>
                    <ppx:InOutVariables/>
                    <ppx:InputVariables>
                        <ppx:InputVariable parameterName="start" negated="false">
                            <ppx:ConnectionPointIn>
                                <ppx:RelPosition x="0" y="20"/>
                                <ppx:Connection refConnectionPointOutId="2"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                        <ppx:InputVariable parameterName="estop" negated="false">
                            <ppx:ConnectionPointIn>
                                <ppx:RelPosition x="0" y="40"/>
                                <ppx:Connection refConnectionPointOutId="9"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                    </ppx:InputVariables>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="StartCmd" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="11">
                                <ppx:RelPosition x="140" y="30"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="MyOr" globalId="12">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="1"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="400" y="180"/>
                    <ppx:Size x="120" y="60"/>
                    <ppx:InOutVariables/>
                    <ppx:InputVariables>
                        <ppx:InputVariable parameterName="a" negated="false">
                            <ppx:ConnectionPointIn>
                                <ppx:RelPosition x="0" y="20"/>
                                <ppx:Connection refConnectionPointOutId="4"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                        <ppx:InputVariable parameterName="b" negated="false">
                            <ppx:ConnectionPointIn>
                                <ppx:RelPosition x="0" y="40"/>
                                <ppx:Connection refConnectionPointOutId="9"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                    </ppx:InputVariables>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="MyOr" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="13">
                                <ppx:RelPosition x="120" y="30"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="SrLatch" instanceName="latch" globalId="14">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="2"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="620" y="100"/>
                    <ppx:Size x="120" y="60"/>
                    <ppx:InOutVariables/>
                    <ppx:InputVariables>
                        <ppx:InputVariable parameterName="set" negated="false">
                            <ppx:ConnectionPointIn>
                                <ppx:RelPosition x="0" y="20"/>
                                <ppx:Connection refConnectionPointOutId="11"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                        <ppx:InputVariable parameterName="reset" negated="false">
                            <ppx:ConnectionPointIn>
                                <ppx:RelPosition x="0" y="40"/>
                                <ppx:Connection refConnectionPointOutId="13"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                    </ppx:InputVariables>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="q" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="15">
                                <ppx:RelPosition x="120" y="30"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="running" globalId="16">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="3"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="820" y="120"/>
                    <ppx:Size x="90" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="15"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:FunctionBlock>
