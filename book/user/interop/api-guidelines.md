# API Guidelines

These guidelines are for developers who write a library for IEC 61131-3 applications. They explain which construct to choose for an interface, and why. The [C interface](c-interface.md) chapter explains what each construct becomes in C.


## Choose the POU kind

Use a `FUNCTION` when the result depends only on the arguments, and nothing must survive the call. A function fits well into expressions, because it has a return value. It cannot keep data.

Use a `FUNCTION_BLOCK` when the interface keeps state between calls, for example a timer or a counter. When the state belongs to the caller and not to the library, a function that takes the data as `VAR_IN_OUT` is the other way to reach it.

Never put a `PROGRAM` in a library. A program has one instance, and that instance belongs to the application.


## Parameters

- `VAR_INPUT` is for values that the POU only reads.
- `VAR_IN_OUT` is for data that the POU reads and writes. It is always a pointer to the caller's variable, and a call must always supply it.
- `VAR_OUTPUT` is for a result that the POU writes.

Use `VAR_IN_OUT` instead of a pointer in `VAR_INPUT`. The pointer makes the caller responsible for the address, and it hides the direction of the data in the interface.

> [!NOTE]
> In a `FUNCTION`, a string, array, or struct in `VAR_INPUT` arrives as a pointer, and a Structured Text body copies it before the first statement. A change inside the function does not reach the caller. A body written in another language gets no such copy, so it must treat the pointer as read only. In a `FUNCTION_BLOCK`, such a value is a member of the instance.

A function that must accept arrays of different lengths declares the parameter as `ARRAY[*]`. The compiler always passes such a parameter by reference, whatever block it stands in.


## Results

A `FUNCTION` states its result as its return type. A `FUNCTION_BLOCK` states its results as `VAR_OUTPUT`. Do not return a result through a pointer in `VAR_INPUT`.

A return type can be a string, an array, or a struct. The compiler makes such a result a hidden first parameter, and the caller provides the memory, so a large result needs no output parameter.


## Private members are visible

Every member of a function block is part of its struct, so the user of the library sees the members that you keep for internal use. They stand in the [generated header](headers.md) next to the inputs and the outputs, and an [access modifier](../language/inheritance.md#access-modifiers) does not hide them, because the compiler parses the modifier and ignores it. Structured Text code that reads such a member gets a warning and builds; C code sees no difference at all.

Give these members names that say that they are internal, and document that they are not part of the interface.


## Types

Choose the type that carries the intention of the value:

- A bit sequence belongs in `BYTE`, `WORD`, `DWORD`, or `LWORD`, not in an integer type.
- A time or a date belongs in a time type, not in `LINT` or `LWORD`.
- A pointer belongs in `REF_TO`, not in an integer type. `REF_TO` is the form of the standard, and the compiler checks the type of what you assign to it.
- A text belongs in `STRING` or `WSTRING` with an explicit length. Without a length, the capacity is 80.


## Initialization

A function block that needs setup declares the method `FB_INIT`. The compiler calls it once for each instance, when that instance is created. Declare it without parameters and without a return type, because the compiler calls it with no arguments.

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
