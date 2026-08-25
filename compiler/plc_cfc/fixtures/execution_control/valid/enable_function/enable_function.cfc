<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:Program xmlns:bmx="http://www.bachmann.at/xml/PLC" xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="enable_function">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <bmx:TextDeclaration>PROGRAM enable_function
VAR
    trigger : BOOL;
    a : DINT := 3;
    b : DINT := 4;
    sum : DINT;
    done : BOOL;
END_VAR</bmx:TextDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="a" globalId="1">
                    <ppx:RelPosition x="370" y="140"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="2">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="b" globalId="3">
                    <ppx:RelPosition x="370" y="160"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="4">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="trigger" globalId="5">
                    <ppx:RelPosition x="370" y="100"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="6">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="myAdd" globalId="7">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="0"/>
                        </ppx:Data>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:ExecutionControl>
                                <bmx:En negated="false">
                                    <ppx:ConnectionPointIn>
                                        <ppx:RelPosition x="0" y="10"/>
                                        <ppx:Connection refConnectionPointOutId="6"/>
                                    </ppx:ConnectionPointIn>
                                </bmx:En>
                                <bmx:Eno negated="false">
                                    <ppx:ConnectionPointOut connectionPointOutId="13">
                                        <ppx:RelPosition x="130" y="10"/>
                                    </ppx:ConnectionPointOut>
                                </bmx:Eno>
                            </bmx:ExecutionControl>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="490" y="100"/>
                    <ppx:Size x="130" y="80"/>
                    <ppx:InOutVariables/>
                    <ppx:InputVariables>
                        <ppx:InputVariable parameterName="in1" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="50"/>
<ppx:Connection refConnectionPointOutId="2"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                        <ppx:InputVariable parameterName="in2" negated="false">
                            <ppx:ConnectionPointIn>
<ppx:RelPosition x="0" y="70"/>
<ppx:Connection refConnectionPointOutId="4"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                    </ppx:InputVariables>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="myAdd" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="8">
<ppx:RelPosition x="130" y="50"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                        <ppx:OutputVariable parameterName="myAddDoubled" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="12">
<ppx:RelPosition x="130" y="70"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="sum" globalId="9">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="1"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="650" y="140"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="8"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="done" globalId="10">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="2"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="650" y="100"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="13"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:Program>
