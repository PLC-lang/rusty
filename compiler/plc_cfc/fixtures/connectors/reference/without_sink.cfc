<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:Program xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="program_0">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <textDeclaration>
                <content>PROGRAM program_0
    VAR
        foo, bar: DINT;
    END_VAR
 </content>
            </textDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="foo" globalId="1">
                    <ppx:RelPosition x="350" y="220"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="2">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:CommonObject xsi:type="ppx:Connector" label="x" globalId="5">
                    <ppx:RelPosition x="480" y="220"/>
                    <ppx:Size x="50" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="2"/>
                    </ppx:ConnectionPointIn>
                </ppx:CommonObject>
                <ppx:CommonObject xsi:type="ppx:Continuation" label="x" globalId="6">
                    <ppx:RelPosition x="600" y="220"/>
                    <ppx:Size x="60" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="4">
                        <ppx:RelPosition x="60" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:CommonObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:Program>
