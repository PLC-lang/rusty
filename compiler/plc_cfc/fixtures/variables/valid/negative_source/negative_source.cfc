<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:Function xmlns:bmx="http://www.bachmann.at/xml/PLC" xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="negative_source">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <bmx:TextDeclaration>FUNCTION negative_source : INT
VAR
    foo : DINT;
    a, b, c : DINT;
END_VAR</bmx:TextDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="-foo" globalId="1">
                    <ppx:RelPosition x="250" y="140"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="2">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="a" globalId="3">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="0"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="360" y="140"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="2"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="-(foo)" globalId="4">
                    <ppx:RelPosition x="250" y="180"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="5">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="b" globalId="6">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="1"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="360" y="180"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="5"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="(-foo)" globalId="7">
                    <ppx:RelPosition x="250" y="220"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="8">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="c" globalId="9">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="2"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="360" y="220"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="8"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:Function>
