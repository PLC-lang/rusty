<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:Program xmlns:bmx="http://www.bachmann.at/xml/PLC" xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="operator_functions">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <bmx:TextDeclaration>PROGRAM operator_functions
VAR
    a, b, c : BOOL;
    x, y : DINT;
    and_result, or_result, xor_result, not_result : BOOL;
    mod_result : DINT;
END_VAR</bmx:TextDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="AND" globalId="6">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="1"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="300" y="50"/>
                    <ppx:Size x="80" y="80"/>
                    <ppx:InOutVariables/>
                    <ppx:InputVariables>
                        <ppx:InputVariable parameterName="IN1" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="30"/>
<ppx:Connection refConnectionPointOutId="1"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                        <ppx:InputVariable parameterName="" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="50"/>
<ppx:Connection refConnectionPointOutId="2"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                        <ppx:InputVariable parameterName="" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="70"/>
<ppx:Connection refConnectionPointOutId="3"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                    </ppx:InputVariables>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="7">
<ppx:RelPosition x="80" y="30"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="OR" globalId="8">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="2"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="300" y="170"/>
                    <ppx:Size x="80" y="60"/>
                    <ppx:InOutVariables/>
                    <ppx:InputVariables>
                        <ppx:InputVariable parameterName="IN1" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="30"/>
<ppx:Connection refConnectionPointOutId="1"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                        <ppx:InputVariable parameterName="" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="50"/>
<ppx:Connection refConnectionPointOutId="2"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                    </ppx:InputVariables>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="9">
<ppx:RelPosition x="80" y="30"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="XOR" globalId="10">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="3"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="300" y="290"/>
                    <ppx:Size x="80" y="60"/>
                    <ppx:InOutVariables/>
                    <ppx:InputVariables>
                        <ppx:InputVariable parameterName="IN1" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="30"/>
<ppx:Connection refConnectionPointOutId="1"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                        <ppx:InputVariable parameterName="" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="50"/>
<ppx:Connection refConnectionPointOutId="2"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                    </ppx:InputVariables>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="11">
<ppx:RelPosition x="80" y="30"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="MOD" globalId="12">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="4"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="300" y="410"/>
                    <ppx:Size x="80" y="60"/>
                    <ppx:InOutVariables/>
                    <ppx:InputVariables>
                        <ppx:InputVariable parameterName="IN1" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="30"/>
<ppx:Connection refConnectionPointOutId="4"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                        <ppx:InputVariable parameterName="IN2" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="50"/>
<ppx:Connection refConnectionPointOutId="5"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                    </ppx:InputVariables>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="13">
<ppx:RelPosition x="80" y="30"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="NOT" globalId="14">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="5"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="300" y="530"/>
                    <ppx:Size x="80" y="40"/>
                    <ppx:InOutVariables/>
                    <ppx:InputVariables>
                        <ppx:InputVariable parameterName="IN" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="30"/>
<ppx:Connection refConnectionPointOutId="1"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                    </ppx:InputVariables>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="15">
<ppx:RelPosition x="80" y="30"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="a" globalId="16">
                    <ppx:RelPosition x="180" y="50"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="1">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="b" globalId="17">
                    <ppx:RelPosition x="180" y="90"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="2">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="c" globalId="18">
                    <ppx:RelPosition x="180" y="130"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="3">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="x" globalId="19">
                    <ppx:RelPosition x="180" y="170"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="4">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="y" globalId="20">
                    <ppx:RelPosition x="180" y="210"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="5">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="and_result" globalId="21">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="6"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="420" y="70"/>
                    <ppx:Size x="100" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="7"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="or_result" globalId="22">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="7"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="420" y="190"/>
                    <ppx:Size x="100" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="9"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="xor_result" globalId="23">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="8"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="420" y="310"/>
                    <ppx:Size x="100" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="11"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="mod_result" globalId="24">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="9"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="420" y="430"/>
                    <ppx:Size x="100" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="13"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="not_result" globalId="25">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="10"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="420" y="550"/>
                    <ppx:Size x="100" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="15"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:Program>
