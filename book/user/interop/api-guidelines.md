# API Guidelines

These guidelines are for developers who write a library for IEC 61131-3 applications. They explain which construct to choose for an interface, and why. The [C interface](c-interface.md) chapter explains what each construct becomes in C.


## Choose the POU kind

| Kind | Use it when |
|---|---|
| `FUNCTION` | The result depends only on the arguments, and nothing must survive the call |
| `FUNCTION_BLOCK` | The interface keeps state between calls, for example a timer or a counter |
| `PROGRAM` | Never in a library. A program has one static instance, which belongs to the application |

A function fits well into expressions, because it has a return value. It cannot keep data. If you need state, use a function block, or let the caller own the data and pass it as `VAR_IN_OUT`.


## Parameters

- `VAR_INPUT` is for values that the POU only reads.
- `VAR_IN_OUT` is for data that the POU reads and writes. It is always a pointer to the caller's variable, and it is always necessary in a call.
- `VAR_OUTPUT` is for results of a function block.

Use `VAR_IN_OUT` instead of a pointer in `VAR_INPUT`. The pointer makes the caller responsible for the address, and it hides the direction of the data in the interface.

> [!NOTE]
> In a `FUNCTION`, a string, array, or struct in `VAR_INPUT` is passed as a pointer to a copy. Changes inside the function do not reach the caller. In a `FUNCTION_BLOCK`, such a value is stored in the instance.

A function that takes an array of any size declares it as `ARRAY[*]`.


## Results

A `FUNCTION` states its result as its return type. A `FUNCTION_BLOCK` states its results as `VAR_OUTPUT`. Do not return a result through a pointer in `VAR_INPUT`.

A return type can be a string, an array, or a struct. The caller then provides the memory, so the result is not limited to one machine word.


## Private members are visible

Every member of a function block is part of its struct, so the user of the library sees the members that you keep for internal use. Give them names that say that they are internal, and document that they are not part of the interface.


## Types

Choose the type that carries the intention of the value:

- A bit sequence belongs in `BYTE`, `WORD`, `DWORD`, or `LWORD`, not in an integer type.
- A time or a date belongs in a time type, not in `LINT` or `LWORD`.
- A pointer belongs in `REF_TO`, not in `LWORD`. `REF_TO` is the form of the standard; `POINTER TO` is the older form.
- A text belongs in `STRING` or `WSTRING` with an explicit length.


## Initialization

A function block that needs setup declares the method `FB_INIT`. The compiler calls it when the instance is created, before any other code runs. In the current implementation, `FB_INIT` takes no parameters and returns nothing.

```iecst
FUNCTION_BLOCK Counter
    VAR
        current: DINT;
    END_VAR

    METHOD FB_INIT
        current := 1;
    END_METHOD
END_FUNCTION_BLOCK
```

For a library whose implementation is not Structured Text, provide the symbol `<FunctionBlock>__FB_INIT` and declare the method in the interface file. The [C interface](c-interface.md) chapter has the symbol names and the build options.

## What's next

That is everything the guide covers. The [reference](../reference/README.md) holds the complete lists: every option, every project key, every construct, and every error code.
