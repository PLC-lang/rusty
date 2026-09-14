# Graphical Programs

A Continuous Function Chart (CFC) holds the body of a POU as a diagram instead of text. You draw it in an engineering tool, which saves it as XML in the PLCopen exchange format, and the compiler reads that file like any other source.

```bash
plc chart.cfc library.st main.st -o app --linker=cc
```

A chart and a text file work together in both directions: the chart calls what the text declares, and the text calls the POU that the chart holds.


## What a chart contains

Two parts. The declaration is Structured Text, written in the declaration editor of the tool and stored as text in the file:

```iecst
PROGRAM Mixer
    VAR
        left: Counter;
        right: Counter;
        outA, outB: DINT;
    END_VAR
```

The body is the network: elements and the wires between them. The compiler turns each element into a statement, so a chart is a program in the same language as everything else in this guide.

| Element | What it does |
|---|---|
| Input | Reads a variable or a literal and feeds it into a wire |
| Output | Writes the value of its wire into a variable |
| Block | Calls a function, a function block instance, a program, or an action |
| Connector and continuation | A named break in a wire, to avoid drawing across the whole sheet |
| Jump and label | A conditional jump, and the place it jumps to |
| Return | Leaves the POU when its condition is true |

A small bubble on a pin negates the value that passes it.


## Execution order

The order of the statements is the order of the **evaluation priority** that you give the elements in the tool. It is not the order of the wires and not the position on the sheet:

```
        Add (0)                Scale (1)
 a --> | in1   out | --+--> | in   out | --> result (2)
 b --> | in2       |   |
```

The numbers in parentheses are the priorities. An element without a priority runs after every element that has one, in the order in which the file stores it.

This matters when two blocks write the same variable, or when one block reads a value that another one produces: the priority decides what happens first, so give every element that has a side effect a priority that you chose.


## Calls

A block that calls a **function** has no memory, so every output that a wire reads is stored in a hidden variable during the call. An input that no wire feeds uses the default value of the parameter.

A block that calls a **function block instance**, a program, or an action keeps its outputs in the instance, and a wire that reads such an output reads the member afterwards. Two programs that read each other are therefore allowed: each one sees the value of the previous evaluation.


## EN and ENO

A block can carry two more pins. `EN` is a condition: the call runs only when the value on that pin is true. `ENO` reports the same condition to the next block, which is how a chain of blocks is switched on and off together.

When `EN` is false, the call does not run, and the outputs of the block keep the values they had.


## What the compiler reports

A chart has no lines and columns, so a diagnostic names the element instead, for example `mixer.cfc: Block 6`, where the number is the identifier that the tool gave the element.

| Code | Meaning |
|---|---|
| `E081` | Two connectors use the same label |
| `E082` | A continuation without a connector |
| `E083` | An expression where only a name or a literal is allowed |
| `E084` | An element that is placed but not wired |
| `E085` | A return without a condition |
| `E086` | A connector without an input |
| `E142` | A jump to a label that does not exist |
| `E143` | A label that no jump uses |
| `E144` | A label that is defined twice |
| `E145` | A jump without a condition |
| `E146` | A block whose type the project does not declare |
| `E147` | A wired output that the callee does not have |
| `E149` | A generic block whose type could not be decided |
| `E152` | A block with execution control whose `EN` is not wired |
| `E153` | An `ENO` chain that leads back to itself |
| `E154` | A negation on a reference assignment |
| `E155` | A block with two unnamed return pins |


## What's next

That is the language. The next chapter explains the compiler as a tool: [building](../building/README.md).
