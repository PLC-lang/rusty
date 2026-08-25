<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:Program xmlns:bmx="http://www.bachmann.at/xml/PLC" xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="connector_routing">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <bmx:TextDeclaration>PROGRAM connector_routing
VAR
    inst : counter;
    trigger : BOOL;
    localIn : DINT;
    localOut : DINT;
    done : BOOL;
END_VAR</bmx:TextDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="trigger" globalId="1">
                    <ppx:RelPosition x="250" y="100"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="2">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:CommonObject xsi:type="ppx:Connector" label="x" globalId="3">
                    <ppx:RelPosition x="360" y="100"/>
                    <ppx:Size x="50" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="2"/>
                    </ppx:ConnectionPointIn>
                </ppx:CommonObject>
                <ppx:CommonObject xsi:type="ppx:Continuation" label="x" globalId="4">
                    <ppx:RelPosition x="420" y="100"/>
                    <ppx:Size x="60" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="5">
                        <ppx:RelPosition x="60" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:CommonObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="localIn" globalId="7">
                    <ppx:RelPosition x="380" y="160"/>
                    <ppx:Size x="90" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="8">
                        <ppx:RelPosition x="90" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="counter" instanceName="inst" globalId="6">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="0"/>
                        </ppx:Data>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:ExecutionControl>
                                <bmx:En negated="false">
                                    <ppx:ConnectionPointIn>
                                        <ppx:RelPosition x="0" y="10"/>
                                        <ppx:Connection refConnectionPointOutId="5"/>
                                    </ppx:ConnectionPointIn>
                                </bmx:En>
                                <bmx:Eno negated="false">
                                    <ppx:ConnectionPointOut connectionPointOutId="9">
                                        <ppx:RelPosition x="130" y="10"/>
                                    </ppx:ConnectionPointOut>
                                </bmx:Eno>
                            </bmx:ExecutionControl>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="520" y="120"/>
                    <ppx:Size x="130" y="60"/>
                    <ppx:InOutVariables/>
                    <ppx:InputVariables>
                        <ppx:InputVariable parameterName="in" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="50"/>
<ppx:Connection refConnectionPointOutId="8"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                    </ppx:InputVariables>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="out" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="10">
<ppx:RelPosition x="130" y="50"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:CommonObject xsi:type="ppx:Connector" label="y" globalId="11">
                    <ppx:RelPosition x="680" y="120"/>
                    <ppx:Size x="50" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="9"/>
                    </ppx:ConnectionPointIn>
                </ppx:CommonObject>
                <ppx:CommonObject xsi:type="ppx:Continuation" label="y" globalId="12">
                    <ppx:RelPosition x="740" y="120"/>
                    <ppx:Size x="60" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="13">
                        <ppx:RelPosition x="60" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:CommonObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="localOut" globalId="14">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="1"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="700" y="170"/>
                    <ppx:Size x="90" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="10"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="done" globalId="15">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="2"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="820" y="120"/>
                    <ppx:Size x="90" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="13"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:Program>
