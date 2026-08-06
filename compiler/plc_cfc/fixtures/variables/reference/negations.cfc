<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:Program xmlns:bmx="http://www.bachmann.at/xml/PLC" xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="generic_unresolved">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <bmx:TextDeclaration>PROGRAM generic_unresolved
VAR
    acc : DINT;
END_VAR  </bmx:TextDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="a1" globalId="10">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation outNegated="true"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="70" y="-70"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="11">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="b1" globalId="12">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="0"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="220" y="-70"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="11"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="a2" globalId="14">
                    <ppx:RelPosition x="70" y="-40"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="15">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="b2" globalId="16">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="1"/>
                        </ppx:Data>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation inNegated="true"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="220" y="-40"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="15"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="a3" globalId="18">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation outNegated="true"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="70" y="-10"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="19">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="b3" globalId="20">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="2"/>
                        </ppx:Data>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation inNegated="true"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="220" y="-10"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="19"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="bmx:CfcJump" targetLabel="jmp1" globalId="29">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation inNegated="false"/>
                        </ppx:Data>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="3"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="220" y="90"/>
                    <ppx:Size x="90" y="20"/>
                    <bmx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="31"/>
                    </bmx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="a4" globalId="30">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation outNegated="true"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="70" y="90"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="31">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="bmx:CfcLabel" label="jmp1" globalId="32">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="4"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="340" y="90"/>
                    <ppx:Size x="80" y="20"/>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="a5" globalId="33">
                    <ppx:RelPosition x="70" y="120"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="36">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="bmx:CfcJump" targetLabel="jmp2" globalId="38">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation inNegated="true"/>
                        </ppx:Data>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="5"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="220" y="120"/>
                    <ppx:Size x="90" y="20"/>
                    <bmx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="36"/>
                    </bmx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="bmx:CfcLabel" label="jmp2" globalId="39">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="6"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="340" y="120"/>
                    <ppx:Size x="80" y="20"/>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="a6" globalId="40">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation outNegated="true"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="70" y="150"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="41">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="bmx:CfcJump" targetLabel="jmp3" globalId="42">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation inNegated="true"/>
                        </ppx:Data>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="7"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="220" y="150"/>
                    <ppx:Size x="90" y="20"/>
                    <bmx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="41"/>
                    </bmx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="bmx:CfcLabel" label="jmp3" globalId="43">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="8"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="340" y="150"/>
                    <ppx:Size x="80" y="20"/>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="a7" globalId="44">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation outNegated="true"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="70" y="250"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="45">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:Return" globalId="46">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation inNegated="false"/>
                        </ppx:Data>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="9"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="230" y="250"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="45"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="a8" globalId="47">
                    <ppx:RelPosition x="70" y="280"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="48">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:Return" globalId="49">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation inNegated="true"/>
                        </ppx:Data>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="10"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="230" y="280"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="48"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="a9" globalId="50">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation outNegated="true"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="70" y="310"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="51">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:Return" globalId="52">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation inNegated="true"/>
                        </ppx:Data>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="11"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="230" y="310"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="51"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:CommonObject xsi:type="ppx:Comment" globalId="53">
                    <ppx:RelPosition x="70" y="-100"/>
                    <ppx:Size x="220" y="20"/>
                    <ppx:Content xsi:type="ppx:SimpleText">Assignments (Data Sinks &amp; Sources)</ppx:Content>
                </ppx:CommonObject>
                <ppx:CommonObject xsi:type="ppx:Comment" globalId="54">
                    <ppx:RelPosition x="70" y="60"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:Content xsi:type="ppx:SimpleText">Jumps</ppx:Content>
                </ppx:CommonObject>
                <ppx:CommonObject xsi:type="ppx:Comment" globalId="55">
                    <ppx:RelPosition x="70" y="220"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:Content xsi:type="ppx:SimpleText">Returns</ppx:Content>
                </ppx:CommonObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="a10" globalId="57">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation outNegated="true"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="70" y="410"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="58">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:CommonObject xsi:type="ppx:Connector" label="connection1" globalId="59">
                    <ppx:RelPosition x="230" y="410"/>
                    <ppx:Size x="110" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="58"/>
                    </ppx:ConnectionPointIn>
                </ppx:CommonObject>
                <ppx:CommonObject xsi:type="ppx:Comment" globalId="60">
                    <ppx:RelPosition x="70" y="380"/>
                    <ppx:Size x="670" y="20"/>
                    <ppx:Content xsi:type="ppx:SimpleText">Connections (Connections and Continuation elements can not be negated, only the node that is connected to it)</ppx:Content>
                </ppx:CommonObject>
                <ppx:CommonObject xsi:type="ppx:Continuation" label="connection1" globalId="61">
                    <ppx:RelPosition x="400" y="410"/>
                    <ppx:Size x="120" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="62">
                        <ppx:RelPosition x="120" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:CommonObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="b10" globalId="63">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="12"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="570" y="410"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="62"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="a11" globalId="65">
                    <ppx:RelPosition x="70" y="440"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="66">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:CommonObject xsi:type="ppx:Connector" label="connection1" globalId="67">
                    <ppx:RelPosition x="230" y="440"/>
                    <ppx:Size x="110" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="66"/>
                    </ppx:ConnectionPointIn>
                </ppx:CommonObject>
                <ppx:CommonObject xsi:type="ppx:Continuation" label="connection1" globalId="68">
                    <ppx:RelPosition x="400" y="440"/>
                    <ppx:Size x="120" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="69">
                        <ppx:RelPosition x="120" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:CommonObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="b11" globalId="70">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="13"/>
                        </ppx:Data>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation inNegated="true"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="570" y="440"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="69"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="b12" globalId="71">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation outNegated="true"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="70" y="470"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="72">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:CommonObject xsi:type="ppx:Connector" label="connection1" globalId="73">
                    <ppx:RelPosition x="230" y="470"/>
                    <ppx:Size x="110" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="72"/>
                    </ppx:ConnectionPointIn>
                </ppx:CommonObject>
                <ppx:CommonObject xsi:type="ppx:Continuation" label="connection1" globalId="74">
                    <ppx:RelPosition x="400" y="470"/>
                    <ppx:Size x="120" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="75">
                        <ppx:RelPosition x="120" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:CommonObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="b12" globalId="76">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="14"/>
                        </ppx:Data>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <bmx:Negation inNegated="true"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="570" y="470"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="75"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:Program>
