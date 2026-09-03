# External Declarations

A POU marked `{external}` has its implementation somewhere else. The compiler takes the declaration, generates no body, and leaves the symbol for the linker.

```iecst
{external}
FUNCTION log : DINT
    VAR_IN_OUT
        message : STRING[1024];
    END_VAR
    VAR_INPUT
        severity : (Err, Warn, Info) := Info;
    END_VAR
END_FUNCTION
```

`log` can now be called from Structured Text. At link time a function with a compatible signature must exist, otherwise the link fails with an undefined symbol.

The attribute works on `PROGRAM`, `FUNCTION`, and `FUNCTION_BLOCK`.


## Declarations from a file

`-i` includes a whole file as external. The compiler reads its declarations and ignores every body:

```bash
plc main.st -i vendor.st -L/opt/vendor/lib -lvendor -o app --linker=cc
```

Repeat `-i` for more files, and quote a pattern, so that the shell does not expand it into arguments that the compiler cannot read:

```bash
plc main.st -i "/usr/share/plc/include/*.st" -l iec61131std -o app --linker=cc
```


## Call a C function

Give the C function a declaration in Structured Text. The signature must match what C expects.

```c
int min(int a, int b);
```

```iecst
{external}
FUNCTION min : DINT
    VAR_INPUT
        a : DINT;
        b : DINT;
    END_VAR
END_FUNCTION
```

The [C interface](c-interface.md) chapter has the rules that turn a declaration into a C signature.


## Variadic arguments

A parameter of type `...` in the last `VAR_INPUT` block makes the function variadic, like `printf` in C.

```iecst
{external}
FUNCTION printf : DINT
    VAR_INPUT {ref}
        format : STRING;
    END_VAR
    VAR_INPUT
        args : ...;
    END_VAR
END_FUNCTION

FUNCTION main : DINT
    VAR
        tmp : DINT;
    END_VAR

    tmp := 1;
    printf('Value %d, %d, %d$N', tmp, tmp * 10, tmp * 100);
    main := tmp;
END_FUNCTION
```

```bash
plc printer.st -o printer --linker=cc
./printer
```

There are three variadic forms, and they differ in what the callee receives:

| Form | What the callee gets |
|---|---|
| `args : ...` | Every argument, then a null pointer after the last one |
| `args : T...` | Every argument as a `T`, and nothing after them |
| `args : {sized} T...` | The number of arguments, then a pointer to an array of `T` |

The null pointer of the untyped form lets a callee that walks the list until a terminator always find the end, also when the call carries no variadic argument at all. A terminator that the caller writes ends the list first, so the added one stays unread. A callee that reads a fixed number of arguments, such as `printf` with its format string or a function with a count parameter, never sees the terminator.

The typed forms keep exactly the argument list of the caller, so their callees must not expect a terminator.

> [!NOTE]
> Arguments of the untyped form follow the promotion rules of C: values smaller than 32 bits arrive as 32-bit values.

## What's next

A declaration must match what the other side expects. The [next chapter](c-interface.md) gives the C type of every construct.
