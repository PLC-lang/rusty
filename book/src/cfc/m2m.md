# Model-to-Model Conversion

Consider the heavily minified CFC file [`MyProgram.cfc`](m2m.md#example-myprogramcfc), which translates to the CFC chart below.
```
                 x                      MyAdd
            ┌────────────┐        ┌───────────────┐
            │            │        │      exec_id:0│
            │            ├───────►│ a             │               z
            │ local_id:0 │        │ref_local_id:0 │          ┌──────────────┐
            └────────────┘        │               │          │              │
                 y                │               ├─────────►│              │
            ┌────────────┐        │               │          │ref_local_id:2│
            │            │        │               │          └──────────────┘
            │            ├───────►│ b             │
            │ local_id:1 │        │ref_local_id:1 │
            └────────────┘        └───────────────┘
                                     local_id:2
``` 

The initial phase of the transformation process involves streaming the entire input file. During the streaming process, whenever important keywords are encountered, they are directly mapped into a corresponding model structure. For example, when reaching the line `<block localId="3" ...>` within the XML file, we generate a model that can be represented as follows:
```rust
struct Block {
    localId: 3,
    type_name: "MyAdd",
    instance_name: None,
    execution_order_id: 0,
    variables: [
        InputVariable  { ... }, // x
        InputVariable  { ... }, // y
        OutputVariable { ... }, // MyAdd, eventually becoming `z := MyAdd`
    ]
}
```

This structure of the CFC graph corresponds to the ST expression `z := MyAdd(x, y);`. 
For more information referer to the internal [`plc_xml`](https://github.com/PLC-lang/rusty/tree/master/compiler/plc_xml) crate.

<!-- ### Examples
`MyAdd.st`
```smalltalk
FUNCTION MyAdd : DINT
    VAR_INPUT
        x, y : DINT;
    END_VAR

    MyAdd := x + y;
END_FUNCTION
``` -->

## Example: `MyProgram.cfc`
```xml
<pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="myProgram" pouType="program">
    <content>
        PROGRAM myProgram
            VAR
                x, y, z : DINT;
            END_VAR
    </content>
    <body>
        <FBD>
            <inVariable localId="1" height="20" width="80" negated="false">
                <expression>x</expression>
            </inVariable>
            <inVariable localId="2" height="20" width="80" negated="false">
                <expression>y</expression>
            </inVariable>
            <block localId="3" width="74" height="60" typeName="MyAdd" executionOrderId="0">
                <inputVariables>
                    <variable formalParameter="x" negated="false">
                        <connectionPointIn>
                            <connection refLocalId="1"/>
                        </connectionPointIn>
                    </variable>
                    <variable formalParameter="y" negated="false">
                        <connectionPointIn>
                            <connection refLocalId="2"/>
                        </connectionPointIn>
                    </variable>
                </inputVariables>
                <outputVariables>
                    </variable formalParameter="MyAdd" negated="false">
                </outputVariables>
            </block>
            <outVariable localId="4" height="20" width="80" executionOrderId="1" negated="false" storage="none">
                <position x="680" y="160"/>
                <connectionPointIn>
                    <connection refLocalId="3" formalParameter="MyAdd"/>
                </connectionPointIn>
                <expression>z</expression>
            </outVariable>
        </FBD>
    </body>
</pou>
```