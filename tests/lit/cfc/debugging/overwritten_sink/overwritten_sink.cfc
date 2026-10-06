<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:Function xmlns:bmx="http://www.bachmann.at/xml/PLC" xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="overwritten_sink">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <bmx:TextDeclaration>FUNCTION overwritten_sink : INT
	VAR
		x1: INT := 4;
		x2: INT := 2;
		t1: TOD := TOD#11:10:11;
		t2: TIME := T#10s;
		result: TOD;
	END_VAR
  </bmx:TextDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="ADD" globalId="1">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="1"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="510" y="120"/>
                    <ppx:Size x="60" y="60"/>
                    <ppx:InOutVariables/>
                    <ppx:InputVariables>
                        <ppx:InputVariable parameterName="IN1" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="30"/>
<ppx:Connection refConnectionPointOutId="3"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                        <ppx:InputVariable parameterName="" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="50"/>
<ppx:Connection refConnectionPointOutId="5"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                    </ppx:InputVariables>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="2">
<ppx:RelPosition x="60" y="10"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="-x1" globalId="4">
                    <ppx:RelPosition x="360" y="140"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="3">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="x2" globalId="6">
                    <ppx:RelPosition x="360" y="160"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="5">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="x2" globalId="7">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="2"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="640" y="120"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="2"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="SUB" globalId="8">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="3"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="510" y="200"/>
                    <ppx:Size x="60" y="60"/>
                    <ppx:InOutVariables/>
                    <ppx:InputVariables>
                        <ppx:InputVariable parameterName="IN1" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="30"/>
<ppx:Connection refConnectionPointOutId="10"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                        <ppx:InputVariable parameterName="IN2" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="50"/>
<ppx:Connection refConnectionPointOutId="12"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                    </ppx:InputVariables>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="9">
<ppx:RelPosition x="60" y="10"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="x1" globalId="11">
                    <ppx:RelPosition x="360" y="220"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="10">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="x2" globalId="13">
                    <ppx:RelPosition x="360" y="240"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="12">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="x2" globalId="14">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="4"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="640" y="200"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="9"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:Function>
