<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:Program xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="mainProgram">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <textDeclaration>
                <content>PROGRAM mainProgram
VAR
	localIn0, localIn1: DINT;
	result: DINT;
END_VAR</content>
            </textDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="SEL" globalId="10">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="2"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="310" y="210"/>
                    <ppx:Size x="80" y="80"/>
                    <ppx:InOutVariables/>
                    <ppx:InputVariables>
                        <ppx:InputVariable parameterName="G" negated="false">
                            <ppx:ConnectionPointIn>
                                <ppx:RelPosition x="0" y="30"/>
                                <ppx:Connection refConnectionPointOutId="11"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                        <ppx:InputVariable parameterName="IN0" negated="false">
                            <ppx:ConnectionPointIn>
                                <ppx:RelPosition x="0" y="50"/>
                                <ppx:Connection refConnectionPointOutId="12"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                        <ppx:InputVariable parameterName="IN1" negated="false">
                            <ppx:ConnectionPointIn>
                                <ppx:RelPosition x="0" y="70"/>
                                <ppx:Connection refConnectionPointOutId="13"/>
                            </ppx:ConnectionPointIn>
                        </ppx:InputVariable>
                    </ppx:InputVariables>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="SEL" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="14">
                                <ppx:RelPosition x="80" y="30"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="result" globalId="15">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="3"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="420" y="230"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="14"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="localIn0" globalId="16">
                    <ppx:RelPosition x="190" y="250"/>
                    <ppx:Size x="90" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="12">
                        <ppx:RelPosition x="90" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="localIn1" globalId="17">
                    <ppx:RelPosition x="190" y="270"/>
                    <ppx:Size x="90" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="13">
                        <ppx:RelPosition x="90" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="TRUE" globalId="18">
                    <ppx:RelPosition x="200" y="230"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="11">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:Program>
