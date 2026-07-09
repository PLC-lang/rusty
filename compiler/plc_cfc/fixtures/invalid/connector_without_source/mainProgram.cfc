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
                <ppx:CommonObject xsi:type="ppx:Connector" label="relay" globalId="1">
                    <ppx:RelPosition x="200" y="60"/>
                    <ppx:Size x="60" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                    </ppx:ConnectionPointIn>
                </ppx:CommonObject>
                <ppx:CommonObject xsi:type="ppx:Continuation" label="relay" globalId="2">
                    <ppx:RelPosition x="200" y="120"/>
                    <ppx:Size x="60" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="10">
                        <ppx:RelPosition x="60" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:CommonObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="result" globalId="3">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="0"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="400" y="120"/>
                    <ppx:Size x="110" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="10"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:Program>
