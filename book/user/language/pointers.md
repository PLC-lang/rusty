# Pointers and References

A pointer holds the address of a variable instead of a value. You need one to share data without copying it, to walk through memory, and to talk to code that was not written in Structured Text.


## Declaring and dereferencing

`REF_TO` is the form of the standard, `POINTER TO` is the older spelling of the same thing:

```iecst
VAR
    value : DINT := 10;
    p : REF_TO DINT;
    q : POINTER TO DINT;
END_VAR

p := REF(value);
q := ADR(value);
```

`REF` and `ADR` both give the address of a variable. `^` reads and writes through the pointer:

```iecst
p^ := p^ + 1;   (* value is now 11 *)
```

A pointer that points nowhere is `NULL`, and a pointer starts as `NULL` until something gives it an address. Nothing checks that at run time: reading through a `NULL` pointer, or through a pointer to a variable that no longer exists, is undefined behavior in the compiled program.

Adding a whole number to a pointer moves it by that many elements, which is how you walk an array:

```iecst
VAR
    values : ARRAY[0..3] OF DINT := [10, 20, 30, 40];
    p : REF_TO DINT;
END_VAR

p := REF(values[0]);
first  := p^;         (* 10 *)
second := (p + 1)^;   (* 20 *)
```


## References

A reference is a pointer that you do not dereference. It is declared with `REFERENCE TO`, it is bound with `REF=`, and afterwards it is used exactly like the variable it points to:

```iecst
VAR
    target : DINT := 5;
    alias : REFERENCE TO DINT;
END_VAR

alias REF= target;
alias := alias + 1;   (* target is now 6 *)
```

Use a reference where the code reads better without `^`, and a pointer where the address itself is the subject.


## Passing data without copying

Most code needs no pointer at all, because the parameter blocks already say how data travels: `VAR_IN_OUT` passes the caller's variable, and `VAR_INPUT {ref}` passes a large value without a copy. Reach for a pointer when neither fits, for example in a structure that refers to another structure, or at the border to C.

```iecst
TYPE Node : STRUCT
        value : DINT;
        next : REF_TO Node;
    END_STRUCT
END_TYPE
```

The [C interface](../interop/c-interface.md) chapter shows what a pointer looks like on the other side.


## What's next

The next chapter reaches the process image and single bits of a value: [hardware access](hardware-access.md).
