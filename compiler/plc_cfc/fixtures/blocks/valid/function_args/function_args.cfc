<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:Program xmlns:bmx="http://www.bachmann.at/xml/PLC" xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="function_args">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <bmx:TextDeclaration>PROGRAM function_args
VAR
    a1, a2, b1, b2 : DINT;
END_VAR</bmx:TextDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="myAdd" globalId="1">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="0"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="1160" y="460"/>
                    <ppx:Size x="130" y="60"/>
                    <ppx:InOutVariables/>
                    <ppx:InputVariables>
                        <ppx:InputVariable parameterName="in1" negated="true">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="30"/>
<ppx:Connection refConnectionPointOutId="5"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                        <ppx:InputVariable parameterName="in2" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="50"/>
<ppx:Connection refConnectionPointOutId="8"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                    </ppx:InputVariables>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="" negated="true">
                            <ppx:ConnectionPointOut connectionPointOutId="2">
<ppx:RelPosition x="130" y="30"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                        <ppx:OutputVariable parameterName="myAddDoubled" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="3">
<ppx:RelPosition x="130" y="50"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="a1" globalId="4">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation outNegated="true"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="1030" y="480"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="5">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="a2" globalId="7">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation outNegated="true"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="1030" y="500"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="8">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="b1" globalId="9">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="1"/>
                        </ppx:Data>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation inNegated="true"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="1340" y="480"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="2"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="b2" globalId="11">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="2"/>
                        </ppx:Data>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation inNegated="true"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="1340" y="500"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="3"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:Program>
