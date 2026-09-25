# Pointers and References

A pointer holds the address of a variable instead of a value. With one you reach the same data from two places, walk through an array element by element, and talk to code that was not written in Structured Text.


## Declaring and dereferencing

`REF_TO` and `POINTER TO` declare the same type, and a value of the one is accepted for the other. They differ in what the compiler checks: it warns about an assignment that would make a `REF_TO` point at another type, and says nothing for a `POINTER TO`.

```iecst
VAR
    value: DINT := 10;
    p: REF_TO DINT;
    q: POINTER TO DINT;
END_VAR

p := REF(value);
q := ADR(value);
```

`REF` and `ADR` both give the address of a variable, and both are [built in](../reference/built-in-functions.md). `REF` keeps the type of that variable, which is what the check above reads. `ADR` gives the address alone, so an assignment of it is never reported.

`^` reads and writes through the pointer:

```iecst
p^ := p^ + 1;   (* value is now 11 *)
```

A pointer that holds no address is `NULL`, and a pointer starts as `NULL` until something gives it an address. Nothing checks a dereference. A program that reads through a `NULL` pointer compiles without a diagnostic and reads address zero: it stops with a segmentation fault, or gives a value that has no meaning.

A declaration can give the address at once, which keeps the pointer out of that state:

```iecst
VAR
    x: DINT := 7;
    p: REF_TO DINT := REF(x);
END_VAR
```

An address is not a constant, so the compiler writes it in code that runs before the body.

Adding a whole number to a pointer moves it by that many elements, not by that many bytes, which is how you walk an array:

```iecst
VAR
    values: ARRAY[0..3] OF DINT := [10, 20, 30, 40];
    p: REF_TO DINT;
    first: DINT;
    second: DINT;
END_VAR

p := REF(values[0]);
first  := p^;         (* 10 *)
second := (p + 1)^;   (* 20 *)
```


## References

A reference is a pointer that you do not dereference. It is declared with `REFERENCE TO`, it is bound with `REF=`, and afterwards it is used exactly like the variable it points to:

```iecst
VAR
    target: DINT := 5;
    alias: REFERENCE TO DINT;
END_VAR

alias REF= target;
alias := alias + 1;   (* target is now 6 *)
```

`REF=` also works in the declaration, as `REF` does for a pointer: `alias: REFERENCE TO DINT REF= target;`. `AT` binds a name to another variable in the same way, and the [hardware access](hardware-access.md) chapter shows it.

Use a reference where the code reads better without `^`, and a pointer where the address itself is the subject.


## Passing data without copying

Both are needed less often than you may expect, because the parameter blocks already say how data travels: `VAR_IN_OUT` passes the caller's variable, and `VAR_INPUT {ref}` passes a large value without a copy. Reach for a pointer when neither fits, for example in a structure that refers to another structure, or at the border to C.

```iecst
TYPE Node:
    STRUCT
        value: DINT;
        next: REF_TO Node;
    END_STRUCT
END_TYPE
```

The [C interface](../interop/c-interface.md) chapter shows what a pointer looks like on the other side.


## What's next

The next chapter reaches the process image and single bits of a value: [hardware access](hardware-access.md).
