# Generic Functions

A function can declare type parameters. A type parameter stands for the type that the call uses, and it has a constraint that says which types are allowed.

```iecst
FUNCTION MAX <T: ANY_ELEMENTARY>: T
    VAR_INPUT
        in1: T;
        in2: T;
    END_VAR
END_FUNCTION
```

A type parameter can be the type of an input, of an output, and of the return value.


## How a call is resolved

A generic function is never called. Every call resolves to a concrete version, and the name of that version is the name of the function and then, for each type parameter, two underscores and the resolved type:

```iecst
x := MAX(aDint, bDint);   (* calls MAX__DINT *)
```

The compiler does not write the body of that version. Either a `FUNCTION` with the resolved name provides it, written in Structured Text, or an object file or library that the linker finds does, for example [one written in C](../interop/calling-c.md).

```iecst
FUNCTION MAX__DINT: DINT
    VAR_INPUT
        in1: DINT;
        in2: DINT;
    END_VAR

    IF in1 > in2 THEN
        MAX__DINT := in1;
    ELSE
        MAX__DINT := in2;
    END_IF
END_FUNCTION
```

When nothing in the project defines the name, the compiler writes an `{external}` declaration for it and leaves the symbol to the linker. If the linker finds nothing either, the build fails with an undefined symbol.

The generic functions that the compiler knows itself work differently. A call to `ABS`, `ADD`, or `SEL` gets no version of its own, because the compiler writes the code at the place of the call. [Built-in Functions](../reference/built-in-functions.md) lists them all.


## Constraints

The constraint is a type nature, for example `ANY_INT`, `ANY_REAL`, `ANY_ELEMENTARY`, or `ANY`. An argument whose type does not have that nature is rejected:

```iecst
FUNCTION Inc <T: ANY_INT>: T
    VAR_INPUT
        v: T;
    END_VAR
END_FUNCTION

FUNCTION main: DINT
    VAR
        value: REAL := 1.0;
    END_VAR

    (* error[E062]: Invalid type nature for generic argument. REAL is no ANY_INT *)
    main := Inc(value);
END_FUNCTION
```

An integer argument for an `ANY_REAL` constraint is the one exception. The compiler converts it to a floating-point type and resolves the call with that type, so a `<T: ANY_REAL>` function called with a `DINT` resolves to its `REAL` version.

`ANY` accepts every type, so a call with a type that has no implementation passes the compiler and fails at the link step:

```iecst
TYPE Point:
    STRUCT
        x, y: DINT;
    END_STRUCT
END_TYPE

FUNCTION main: DINT
    VAR
        s: STRING;
        p: Point;
    END_VAR

    s := TO_STRING(p);   (* undefined symbol TO_STRING__Point at link time *)
END_FUNCTION
```

The standard library uses this for its conversions. `TO_STRING` declares `<T: ANY>` and the library provides an implementation for every type that it supports, such as `TO_STRING__DINT` and `TO_STRING__REAL`.

The natures that a constraint can name are `ANY`, `ANY_DERIVED`, `ANY_ELEMENTARY`, `ANY_MAGNITUDE`, `ANY_NUM`, `ANY_REAL`, `ANY_INT`, `ANY_SIGNED`, `ANY_UNSIGNED`, `ANY_DURATION`, `ANY_BIT`, `ANY_CHARS`, `ANY_STRING`, `ANY_CHAR`, and `ANY_DATE`. Any other name is rejected. They form a tree with `ANY` at the root: `ANY_SIGNED` is part of `ANY_INT`, `ANY_INT` is part of `ANY_NUM`, `ANY_NUM` is part of `ANY_MAGNITUDE`, and `ANY_MAGNITUDE` is part of `ANY_ELEMENTARY`. A constraint accepts every type below it, so `ANY_INT` takes a `DINT` and also a `UDINT`.


## What's next

The next chapter is about working with addresses: [pointers and references](pointers.md).
