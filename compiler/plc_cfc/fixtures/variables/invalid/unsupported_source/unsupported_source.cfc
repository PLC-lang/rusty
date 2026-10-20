<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:Program xmlns:bmx="http://www.bachmann.at/xml/PLC" xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="unsupported_source">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <bmx:TextDeclaration>PROGRAM unsupported_source
VAR
    a, r : DINT;
    arr : ARRAY[0..5] OF DINT;
    grid : ARRAY[0..2, 0..2] OF DINT;
END_VAR</bmx:TextDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="arr[f()]" globalId="1">
                    <ppx:RelPosition x="250" y="140"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="2">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="r" globalId="3">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="1"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="360" y="140"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="2"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="fp()^" globalId="4">
                    <ppx:RelPosition x="250" y="180"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="5">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="r" globalId="6">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="2"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="360" y="180"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="5"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="[f(), 1]" globalId="7">
                    <ppx:RelPosition x="250" y="220"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="8">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="r" globalId="9">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="3"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="360" y="220"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="8"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="a ** 2" globalId="10">
                    <ppx:RelPosition x="250" y="260"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="11">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="r" globalId="12">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="4"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="360" y="260"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="11"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="a := 1" globalId="13">
                    <ppx:RelPosition x="250" y="300"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="14">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="r" globalId="15">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="5"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="360" y="300"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="14"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="1 + (a := 1)" globalId="16">
                    <ppx:RelPosition x="250" y="340"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="17">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="r" globalId="18">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="6"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="360" y="340"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="17"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="a, r" globalId="19">
                    <ppx:RelPosition x="250" y="380"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="20">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="r" globalId="21">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="7"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="360" y="380"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="20"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="1..5" globalId="22">
                    <ppx:RelPosition x="250" y="420"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="23">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="r" globalId="24">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="8"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="360" y="420"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="23"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="*" globalId="25">
                    <ppx:RelPosition x="250" y="460"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="26">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="r" globalId="27">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="9"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="360" y="460"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="26"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="" globalId="28">
                    <ppx:RelPosition x="250" y="500"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="29">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="r" globalId="30">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="10"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="360" y="500"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="29"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="%IX1.0" globalId="31">
                    <ppx:RelPosition x="250" y="540"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="32">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="r" globalId="33">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="11"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="360" y="540"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="32"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="grid[f(), 1]" globalId="34">
                    <ppx:RelPosition x="250" y="580"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="35">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="r" globalId="36">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="12"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="360" y="580"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="35"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="1 + [4294967295(f())]" globalId="37">
                    <ppx:RelPosition x="250" y="620"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="38">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="r" globalId="39">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="13"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="360" y="620"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="38"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:Program>
