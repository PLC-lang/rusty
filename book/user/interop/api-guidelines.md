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

Pass an instance of a function block as `VAR_IN_OUT` as well. In `VAR_INPUT` of a function block, the instance is copied into a member, and a copy of an instance that has `FB_EXIT` is cleaned up a second time, see [Cleanup](#cleanup).


## Results

A `FUNCTION` states its result as its return type. A `FUNCTION_BLOCK` states its results as `VAR_OUTPUT`. Do not return a result through a pointer in `VAR_INPUT`.

A return type can be a string, an array, or a struct. The compiler makes such a result a hidden first parameter, and the caller provides the memory, so a large result needs no output parameter.

Do not return an instance of a function block that has `FB_EXIT`. The function cleans up its result before the caller gets the copy. Let the caller pass the instance as `VAR_IN_OUT`, and fill it.


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


## Cleanup

A function block that holds something it must release declares the method `FB_EXIT`. The compiler calls it once for each instance, when that instance ends. Declare it without parameters and without a return type, like `FB_INIT`.

```iecst
FUNCTION_BLOCK Counter
    VAR
        current: DINT;
    END_VAR

    METHOD FB_EXIT
        current := 0;
    END_METHOD
END_FUNCTION_BLOCK
```

For a library whose implementation is not Structured Text, provide the symbol `<FunctionBlock>__FB_EXIT` and declare the method in the interface file. An application can only call `FB_EXIT` of a library function block when the interface file declares it, so a library that adds the method must ship the new interface file and the destructors that `--constructors-only` writes for it.

`FB_EXIT` is not a destructor as C++ has it. An assignment copies an instance and calls no method, and each of the two variables is cleaned up when it ends. The example uses `LogFile`, a function block of a C library that keeps a file descriptor in the member `fd`:

```c
void LogFile__FB_EXIT(LogFile_type* self) {
    close(self->fd);
}
```

```iecst
VAR_GLOBAL
    audit: LogFile;
    data: LogFile;
END_VAR

FUNCTION snapshot: DINT
    VAR
        copy: LogFile;
    END_VAR

    copy := audit;              // copy.fd = 3, no method is called
END_FUNCTION                    // FB_EXIT of copy: close(3)

FUNCTION main: DINT
    audit.Open('audit.log');    // audit.fd = 3
    snapshot();                 // the file of audit is closed now
    data.Open('data.log');      // data.fd = 3, the number is free again
    audit.Write(42);            // writes 42 to data.log, and reports no error
END_FUNCTION
```

An instance in `VAR_INPUT` of a function block and the result of a function are copies of the same kind. An instance in `VAR_INPUT` of a function is a copy that gets neither `FB_INIT` nor `FB_EXIT`, so nothing releases what a method acquires through it. A library cannot stop a copy, but it stays correct when it follows these rules:

- **Expect `FB_EXIT` for an instance that holds nothing.** It runs for every instance, also when the method that acquires the resource was never called. A member that nothing has set is 0, and 0 is a valid file descriptor: the standard input.
- **Keep a handle in the member, not the resource.** Look the handle up in a table of the library before each use, remove the entry in `FB_EXIT`, and never use a handle value again. The cleanup of a copy then finds no entry, and in the example `audit.Write(42)` returns an error and `data.log` stays untouched.
- **Do not acquire a resource in `FB_INIT`.** The target of an assignment loses it, because the copy overwrites the member. Acquire it in a method that the application calls, as `Open` does.
- **Do not call the `FB_EXIT` of the base.** The destructor of a derived function block calls the derived method and then the method of the base. A call from the derived method runs the base cleanup two times.
- **Let nothing use the instance after `FB_EXIT`.** A local instance lives on the stack, and its memory is used again after the call. A thread or a job that can outlive the call must hold data that the library owns, not the address of the instance.
- **Catch C++ exceptions inside the library.** An exception that leaves a library function passes through the Structured Text code that called it, and no `FB_EXIT` runs for the local instances of the functions it leaves.

The first rule in C, for a library that keeps the descriptor in the member:

```c
void LogFile__FB_INIT(LogFile_type* self) {
    self->fd = -1;              // nothing to release
}

void LogFile__FB_EXIT(LogFile_type* self) {
    if (self->fd < 0) {
        return;
    }
    close(self->fd);
    self->fd = -1;              // a second call for this instance does nothing
}
```


## What's next

That is everything the guide covers. The [reference](../reference/README.md) holds the complete lists: every option, every project key, every construct, and every error code.
