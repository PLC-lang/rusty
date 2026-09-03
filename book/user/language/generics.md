# Generic Functions

A function can declare type parameters. A type parameter stands for the type that the call uses, and it has a constraint that says which types are allowed.

```iecst
FUNCTION MAX <T: ANY_ELEMENTARY> : T
    VAR_INPUT
        in1 : T;
        in2 : T;
    END_VAR
END_FUNCTION
```

A type parameter can be the type of an input, of an output, and of the return value.


## How a call is resolved

A generic function is never called. Every call resolves to a concrete version, and the name of that version is the name of the function, two underscores, and the resolved type:

```iecst
x := MAX(aDint, bDint);   (* calls MAX__DINT *)
```

The compiler does not write the body of that version. One of three things must provide it:

- a `FUNCTION` with the resolved name, written in Structured Text,
- an implementation that the linker finds, for example in C or Rust,
- a builtin of the compiler.

```iecst
FUNCTION MAX__DINT : DINT
    VAR_INPUT
        in1 : DINT;
        in2 : DINT;
    END_VAR

    IF in1 > in2 THEN
        MAX__DINT := in1;
    ELSE
        MAX__DINT := in2;
    END_IF
END_FUNCTION
```

When nothing in the project defines the name, the compiler writes an `{external}` declaration for it and leaves the symbol to the linker. If the linker finds nothing either, the build fails with an undefined symbol.


## Constraints

The constraint is a type nature, for example `ANY_INT`, `ANY_REAL`, `ANY_ELEMENTARY`, or `ANY`. An argument whose type does not have that nature is rejected:

```iecst
FUNCTION Inc <T: ANY_INT> : T
    VAR_INPUT
        v : T;
    END_VAR
END_FUNCTION

FUNCTION main : DINT
    VAR
        value : REAL := 1.0;
    END_VAR

    main := Inc(value);   (* error[E062]: REAL is no ANY_INT *)
END_FUNCTION
```

`ANY` accepts every type, so a call with a type that has no implementation passes the compiler and fails at the link step:

```iecst
TYPE Point : STRUCT x, y : DINT; END_STRUCT END_TYPE

FUNCTION main : DINT
    VAR
        s : STRING;
        p : Point;
    END_VAR

    s := TO_STRING(p);   (* undefined symbol TO_STRING__Point at link time *)
END_FUNCTION
```

The standard library uses this for its conversions: `TO_STRING <T: ANY> : STRING` has an implementation for every type that it supports, such as `TO_STRING__DINT` and `TO_STRING__REAL`.

The natures that a constraint can name are `ANY`, `ANY_DERIVED`, `ANY_ELEMENTARY`, `ANY_MAGNITUDE`, `ANY_NUM`, `ANY_REAL`, `ANY_INT`, `ANY_SIGNED`, `ANY_UNSIGNED`, `ANY_DURATION`, `ANY_BIT`, `ANY_CHARS`, `ANY_STRING`, `ANY_CHAR`, and `ANY_DATE`. They form a tree: `ANY_INT` is part of `ANY_NUM`, which is part of `ANY_MAGNITUDE`, and so on up to `ANY`.


## What's next

The next chapter is about working with addresses: [pointers and references](pointers.md).
