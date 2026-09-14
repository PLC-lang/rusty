# Graphical Programs

A Continuous Function Chart (CFC) holds the body of a POU as a diagram instead of text. You draw it in an engineering tool, which saves it as XML in the PLCopen exchange format, and the compiler reads that file like any other source.

```bash
plc chart.cfc library.st main.st -o app --linker=cc
```

A chart and a text file work together in both directions: the chart calls what the text declares, and the text calls the POU that the chart holds.


## What a chart contains

A chart has two parts. The declaration is Structured Text, written in the declaration editor of the tool and stored as text in the file:

```iecst
PROGRAM Mixer
    VAR
        left: Counter;
        right: Counter;
        outA, outB: DINT;
    END_VAR
```

The body is the network: the elements and the wires between them. The compiler turns the network into a list of statements, so a chart is a program in the same language as everything else in this guide.

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

The wires say where a value goes, not when. The order of the statements is the order of the **evaluation priority** that you give the elements in the tool. It is not the order of the wires and not the position on the sheet:

```
        Add (0)             Scale (1)
 a --> | in1   out | --> | in    out | --> result (2)
 b --> | in2       |
```

The numbers in parentheses are the priorities. An element without a priority runs after every element that has one, in the order in which the file stores it.

This matters when two blocks write the same variable, or when one block reads a value that another one produces: the priority decides what happens first. Give a priority to every element that becomes a statement: an output, a block, a jump, a label, and a return.


## Calls

What a block call does with the values on its pins depends on what the block calls.

A block that calls a **function** has no memory, so every output that a wire reads is stored in a hidden variable during the call. An input that no wire feeds uses the default value of the parameter.

A block that calls a **function block instance**, a program, or an action keeps its outputs in the instance, and a wire that reads such an output reads the member afterwards. Two blocks that read each other are therefore allowed: the priority decides which one runs first, and that one reads the value the other left in the previous evaluation.


## EN and ENO

A block can carry two more pins. `EN` is a condition: the call runs only when the value on that pin is true. `ENO` reports the same condition to the next block, which is how a chain of blocks is switched on and off together.

When `EN` is false, the call does not run, and the outputs of the block keep the values they had.


## What the compiler reports

The compiler checks the drawing before it turns the network into statements. A chart has no lines and columns, so a diagnostic names the element instead:

```
error[E083]: Unsupported CFC expression: `foo + 1`
 = mixer.cfc: Block 6
```

The number is the identifier that the tool gave the element. The message repeats the rule of the table above: an input or an output element holds a variable or a literal, and not an expression.

Other checks are about the routing. A wire must lead somewhere, so a continuation needs a connector of its name, and a connector that something reads needs an input. A name is claimed once, so two connectors with the same label are rejected, and so are two labels with the same name. A jump to a label that no element defines is rejected as well. A label that no jump uses, a jump without a condition, and an element that you placed but never wired are warnings. A return without a condition is rejected, because it could never fire.

A block is checked against what the project declares. Its type must be a POU that the project knows. Every output that a wire reads must be one that the callee declares, and only one output pin may carry the return value. A generic callee needs an input that decides its type. A block with `EN` needs a wire on that pin, and the chain of `ENO` pins behind it must not lead back to the block itself.

The [error code reference](../reference/error-codes.md) has a page per code.


## What's next

That is the language. The next chapter explains the compiler as a tool: [building](../building/README.md).
