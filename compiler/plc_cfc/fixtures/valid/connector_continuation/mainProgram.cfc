<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:Program xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="mainProgram">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <textDeclaration>
                <content>PROGRAM mainProgram
VAR
	result: DINT;
END_VAR
                </content>
            </textDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
                <ppx:CommonObject xsi:type="ppx:Continuation" label="five" globalId="6">
                    <ppx:RelPosition x="600" y="260"/>
                    <ppx:Size x="70" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="7">
                        <ppx:RelPosition x="70" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:CommonObject>
                <ppx:CommonObject xsi:type="ppx:Connector" label="five" globalId="8">
                    <ppx:RelPosition x="400" y="260"/>
                    <ppx:Size x="60" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="12"/>
                    </ppx:ConnectionPointIn>
                </ppx:CommonObject>
                <ppx:FbdObject xsi:type="ppx:Block" typeName="alwaysFive" globalId="13">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="0"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="230" y="240"/>
                    <ppx:Size x="110" y="40"/>
                    <ppx:InOutVariables/>
                    <ppx:InputVariables/>
                    <ppx:OutputVariables>
                        <ppx:OutputVariable parameterName="alwaysFive" negated="false">
                            <ppx:ConnectionPointOut connectionPointOutId="12">
                                <ppx:RelPosition x="110" y="30"/>
                            </ppx:ConnectionPointOut>
                        </ppx:OutputVariable>
                    </ppx:OutputVariables>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="result" globalId="14">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="1"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="760" y="260"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="7"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:Program>
