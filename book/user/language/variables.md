# Variables

A variable holds a value while the program runs. Where you declare it decides how long the value lives and which code can see it.

A variable is declared inside a variable block, with a name, a type, and an optional initial value. Several names can share one declaration:

```iecst
VAR
    count: DINT := 1;
    x, y, z: REAL;
    name: STRING[20] := 'motor';
END_VAR
```

Assignment uses `:=`, and it works in one direction only. `a := b` writes the value of `b` into `a`, never the value of `a` into `b`.


## Where a variable lives

`VAR` inside a POU declares data of that POU. In a function block or a program the data survives the call; in a function it exists for the call only.

`VAR_TEMP` declares data that exists for one call, also in a function block. Use it for a value that you compute and use inside the body and that must not survive.

`VAR_GLOBAL` stands outside every POU and declares data of the whole project:

```iecst
VAR_GLOBAL
    cycleTime: TIME := T#10ms;
END_VAR
```

A global is visible everywhere, and the POU that uses it declares nothing.

The blocks that declare parameters, `VAR_INPUT`, `VAR_OUTPUT`, and `VAR_IN_OUT`, belong to the POU that they are written in. The chapters on [functions](functions.md) and [function blocks](function-blocks.md) explain them.

> [!NOTE]
> `VAR_EXTERNAL` is parsed, and the compiler warns that the block has no effect. It is not necessary, because a global is visible without it.


## Constants

Write `CONSTANT` on the block and every variable in it becomes a constant. An assignment to one is rejected:

```iecst
VAR_GLOBAL CONSTANT
    MAX_SIZE: INT := 99;
    MIN_LEN: INT := 1;
END_VAR
```

Constants are the way to give a name to a number that appears in declarations, because a constant can be used where the compiler needs a value before the program runs, for example in the bounds of an array.


## Retained variables

A control system loses its memory when it loses power. `RETAIN` marks the variables that must survive that:

```iecst
PROGRAM Main
    VAR RETAIN
        partsProduced: DINT;
    END_VAR
    VAR NON_RETAIN
        scratch: DINT;
    END_VAR
END_PROGRAM
```

The compiler puts retained variables into a section of the binary that is called `.retain`, and leaves the rest to the target. `NON_RETAIN` states the normal behavior, which is also the default.

> [!NOTE]
> `RETAIN` needs explicit handling in the runtime that manages the project. The compiler only marks the storage. It does not save the section, it does not restore it, and the start-up code writes the declared initial value into the section at every start.


## Initial values

An initial value is computed while the program is compiled, so it can use literals, constants, and expressions of them, but nothing that is only known while the program runs:

```iecst
VAR_GLOBAL CONSTANT
    MIN_LEN: INT := 1;
    MAX_LEN: INT := 100;
    SIZE: INT := MAX_LEN - MIN_LEN;
END_VAR
```

A variable without an initial value is not undefined. It gets the initial value of its type if the type has one, and zero otherwise: `0` for numbers, `FALSE` for `BOOL`, the empty string for text, and the same rule for every element of an array and every member of a struct.

An array takes a list, a struct takes its members by name:

```iecst
VAR
    values: ARRAY[0..4] OF DINT := [1, 2, 3, 4, 5];
    origin: Point := (x := 0, y := 0);
END_VAR
```

A list that is shorter than the array fills the rest with the default value of the element type, and the compiler warns so that you notice.

Values that are addresses, such as `REF(x)`, are not constants and cannot be written into the static data of a program. Such a variable starts as a null pointer, and the compiler emits start-up code that sets it before the first call of your code. The [pointers](pointers.md) chapter explains `REF` and the types that hold an address.

### Network Publish Mode

Sysmac Studio publishes global variables to the network. Each variable carries a `networkPublish`attribute in the generated XML. The default is `DoNotPublish`.

To set the mode, write a `network_publish` pragma directly before a `VAR_GLOBAL` block. The mode applies to every variable in that block:

```iecst
{network_publish := 'Input'}
VAR_GLOBAL
    speed : INT;
    torque : INT;
END_VAR
```

The accepted modes are `DoNotPublish`, `PublishOnly`, `Input` and `Output`. The mode name is not case sensitive. The pragma applies to one block only. A later `VAR_GLOBAL` block without its own pragma goes back to `DoNotPublish`:

```iecst
{network_publish := 'Output'}
VAR_GLOBAL
    published : INT;
END_VAR

VAR_GLOBAL
    notPublished : INT;
END_VAR
```

Local variables are not published, so the pragma has no effect on a `VAR` block inside a POU. An unknown mode name gives warning `E024`, and the block falls back to `DoNotPublish`.

## What's next

Variables need types. The next chapter introduces the [basic types](basic-types.md): numbers, bits, and truth values.
