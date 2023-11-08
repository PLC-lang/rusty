# Model-to-Model Conversion

As previously mentioned, the lexical and parsing phases are replaced by a model-to-model conversion process which consists of two steps:
1. Transform the input file (XML) into a data-model
2. Transform the data-model into an AST

## XML to Data-Model
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

The initial phase of the transformation process involves streaming the entire input file.
During the streaming process, whenever important keywords such as `block` are encountered, they are directly mapped into a corresponding model structure. 
For example, when reaching the line `<block localId="3" ...>` within the XML file, we generate a model that can be represented as follows:
```rust
struct Block {
    localId: 2,
    type_name: "MyAdd",
    instance_name: None,
    execution_order_id: 0,
    variables: [
        InputVariable  { ... }, // x, with localId = 0
        InputVariable  { ... }, // y, with localId = 1
        OutputVariable { ... }, // MyAdd eventually becoming `z := MyAdd`, with z having a localId = 2
    ]
}
```

This is process is repeated for every element defined in the [model](https://github.com/PLC-lang/rusty/tree/master/compiler/plc_xml/src/model) folder, and once done is fed into the next phase of the transformation process.

[//]: # (This structure of the CFC graph corresponds to the ST expression `z := MyAdd&#40;x, y&#41;;`. )
[//]: # (For more information referer to the internal [`plc_xml`]&#40;https://github.com/PLC-lang/rusty/tree/master/compiler/plc_xml&#41; crate.)

## Data-Model to AST


## Appendix
### MyProgram.cfc
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

### MyAdd.st
```smalltalk
FUNCTION MyAdd : DINT
    VAR_INPUT
        x, y : DINT;
    END_VAR

    MyAdd := x + y;
END_FUNCTION
```
