<?xml version="1.0" encoding="UTF-8"?>
<Program xmlns="www.iec.ch/public/TC65SC65BWG7TF10"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         name="myMain">
    <AddData>
        <Data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
            <textDeclaration>
                <content>
PROGRAM myMain
VAR
    localA: INT := 10;
    localB: INT := 20;
    localResult: INT := 0;
END_VAR
                </content>
            </textDeclaration>
        </Data>
    </AddData>
    <MainBody>
        <BodyContent xsi:type="FBD">
            <Network xsi:type="FbdNetwork" evaluationOrder="1">
                <FbdObject xsi:type="DataSource" globalId="1" identifier="localA">
                    <ConnectionPointOut connectionPointOutId="1"/>
                </FbdObject>
                <FbdObject xsi:type="DataSource" globalId="2" identifier="localB">
                    <ConnectionPointOut connectionPointOutId="2"/>
                </FbdObject>
                <FbdObject xsi:type="Block" globalId="3" typeName="myAdd">
                    <InputVariables>
                        <InputVariable parameterName="x">
                            <ConnectionPointIn>
                                <Connection refConnectionPointOutId="1"/>
                            </ConnectionPointIn>
                        </InputVariable>
                        <InputVariable parameterName="y">
                            <ConnectionPointIn>
                                <Connection refConnectionPointOutId="2"/>
                            </ConnectionPointIn>
                        </InputVariable>
                    </InputVariables>
                    <OutputVariables>
                        <OutputVariable parameterName="myAdd">
                            <ConnectionPointOut connectionPointOutId="3"/>
                        </OutputVariable>
                    </OutputVariables>
                </FbdObject>
                <FbdObject xsi:type="DataSink" globalId="4" identifier="localResult">
                    <ConnectionPointIn>
                        <Connection refConnectionPointOutId="3"/>
                    </ConnectionPointIn>
                </FbdObject>
            </Network>
        </BodyContent>
    </MainBody>
</Program>
