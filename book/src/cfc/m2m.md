# Model-to-Model Conversion

As previously mentioned, the lexical and parsing phases are replaced by a model-to-model conversion process which consists of two steps:
1. Transform the input file (XML) into a data-model
2. Transform the data-model into an AST

## XML to Data-Model

Consider the heavily minified CFC file [`MyProgram.cfc`](m2m.md#myprogramcfc), which translates to the CFC chart below.
```
                   x                      MyAdd
            ┌─────────────┐        ┌─────────────────┐
            │             │        │    exec_id:0    │
            │             ├───────►│ a               │                 z
            │ local_id: 0 │        │ ref_local_id: 0 │          ┌──────────────┐
            └─────────────┘        │                 │          │  exec_id: 1  │
                   y               │                 ├─────────►│              │
            ┌─────────────┐        │                 │          │ref_local_id:2│
            │             │        │                 │          └──────────────┘
            │             ├───────►│ b               │             local_id: 3
            │ local_id:1  │        │ ref_local_id: 1 │
            └─────────────┘        └─────────────────┘
                                       local_id: 2
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

This is process is repeated for every element in the input file which has a corresponding model implementation. For more information on implementation details, see the [model](https://github.com/PLC-lang/rusty/tree/master/compiler/plc_xml/src/model) folder.

Since the CFC programming language utilizes blocks and their interconnections to establish the program's logic flow,
with the sequencing of block execution and inter-block links represented through corresponding `localId`, `refLocalId` and `excutionOrderId`,
we have to order each element by their execution ID before proceeding to the next phase. 
Otherwise the generated AST statements would be out of order and hence semantically incorrect.

## Data-Model to AST
The final part of the model-to-model transformation takes the input from the previous step and transforms it into an AST which the compiler pipeline understands and can generate code from.
Consider the previous `block` example - the transformer first encounters the element with the `executionOrderId` of 0, which is a call to `myAdd`.
We then check and transform each parameter, input `a` and `b` corresponding to the variables `x` and `y` respectively. The result of this transformation looks as follows:

```Rust
CallStatement { 
    operator: myAdd, 
    parameters: [x, y] 
}
```

   Next, we process the element with an `executionOrderId` of 1, which corresponds to an assignment of the previous call's result to z. This update modifies the generated AST as follows:

```Rust
AssignmentStatement {
    left: z, 
    right: CallStatement {
        operator: myAdd,
        parameters: [x, y]
    }
}
```

While this explanation covers the handling of blocks and variables, there are other elements (e.g. control-flow), that are not discussed here. For more information on implementation details, see [`plc_xml/src/xml_parser`](https://github.com/PLC-lang/rusty/tree/master/compiler/plc_xml/src/xml_parser).

Finally, after transforming all elements into their respective AST statements, the result is passed to the indexer and subsequently enters the next stages of the compiler pipeline, as described in the [architecture documentation](../arch/architecture.md#rusty-frontend-architecture)).

## Appendix
### MyAdd.st
```smalltalk
FUNCTION MyAdd : DINT
    VAR_INPUT
        x, y : DINT;
    END_VAR

    MyAdd := x + y;
END_FUNCTION
```

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
