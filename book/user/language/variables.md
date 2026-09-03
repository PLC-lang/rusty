# Variables

A variable is declared inside a variable block, with a name, a type, and an optional initial value. Several names can share one declaration:

```iecst
VAR
    count : DINT := 1;
    x, y, z : REAL;
    name : STRING[20] := 'motor';
END_VAR
```

Assignment uses `:=`, and it works in one direction only. `a := b` writes the value of `b` into `a`; nothing in the language assigns the other way around.


## Where a variable lives

`VAR` inside a POU declares data of that POU. In a function block or a program the data survives the call; in a function it exists for the call only.

`VAR_TEMP` declares data that exists for one call, also in a function block. Use it for a value that you compute and use inside the body and that must not survive.

`VAR_GLOBAL` stands outside every POU and declares data of the whole project:

```iecst
VAR_GLOBAL
    cycleTime : TIME := T#10ms;
END_VAR
```

A global is visible everywhere, and the POU that uses it declares nothing.

The blocks that declare parameters, `VAR_INPUT`, `VAR_OUTPUT`, and `VAR_IN_OUT`, belong to the POU that they are written in. The chapters on [functions](functions.md) and [function blocks](function-blocks.md) explain them.

> [!NOTE]
> `VAR_EXTERNAL` is parsed and has no effect. The compiler reports `E106`. It is not necessary, because a global is visible without it.


## Constants

Write `CONSTANT` on the block and every variable in it becomes a constant. An assignment to one is reported as `E036`:

```iecst
VAR_GLOBAL CONSTANT
    MAX_SIZE : INT := 99;
    MIN_LEN  : INT := 1;
END_VAR
```

Constants are the way to give a name to a number that appears in declarations, because a constant can be used where the compiler needs a value before the program runs, for example in the bounds of an array.


## Retained variables

A control system loses its memory when it loses power. `RETAIN` marks the variables that must survive that:

```iecst
PROGRAM Main
    VAR RETAIN
        partsProduced : DINT;
    END_VAR
    VAR NON_RETAIN
        scratch : DINT;
    END_VAR
END_PROGRAM
```

The compiler puts retained variables into a separate section of the binary, and the runtime on the target provides the persistent storage for that section. `NON_RETAIN` states the normal behavior, which is also the default.


## Initial values

An initial value is computed while the program is compiled, so it can use literals, constants, and expressions of them, but nothing that is only known while the program runs:

```iecst
VAR_GLOBAL CONSTANT
    MIN_LEN : INT := 1;
    MAX_LEN : INT := 100;
    SIZE    : INT := MAX_LEN - MIN_LEN;
END_VAR
```

A variable without an initial value is not undefined. It gets the initial value of its type if the type has one, and zero otherwise: `0` for numbers, `FALSE` for `BOOL`, the empty string for text, and the same rule for every element of an array and every member of a struct.

An array takes a list, a struct takes its members by name:

```iecst
VAR
    values : ARRAY[0..4] OF DINT := [1, 2, 3, 4, 5];
    origin : Point := (x := 0, y := 0);
END_VAR
```

A list that is shorter than the array fills the rest with the default value of the element type, and the compiler reports `E127` so that you notice.

Values that are addresses, such as `REF(x)`, are not constants and cannot be written into the static data of a program. The compiler collects them and sets them before the first call of your code; the [pointers](pointers.md) chapter comes back to this.


## What's next

Variables need types. The next chapter introduces the [basic types](basic-types.md): numbers, bits, and truth values.
