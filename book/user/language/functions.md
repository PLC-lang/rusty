# Functions

A function computes a result from its arguments and forgets everything else. Two calls with the same arguments give the same answer, as long as the function does not read a global variable.

```iecst
FUNCTION Scale: DINT
    VAR_INPUT
        value: DINT;
        factor: DINT;
    END_VAR

    Scale := value * factor;
END_FUNCTION
```

The type after the colon is the result type. Inside the body, the name of the function is the result: you assign to it, and the last value you assigned is what the caller gets. A function without a result type returns nothing, and a call of it is a statement.


## Nothing survives a call

Every local variable of a function is temporary, also in a `VAR` block:

```iecst
FUNCTION Counter: DINT
    VAR
        n: DINT;
    END_VAR

    n := n + 1;
    Counter := n;
END_FUNCTION
```

Both calls of `Counter()` return `1`. When you need the value of the previous call, use a [function block](function-blocks.md).

A function may call itself. The compiler sets no limit on the depth, but the stack of the target does.


## Parameters

Four blocks describe what goes in and what comes out:

```iecst
FUNCTION Measure: DINT
    VAR_INPUT
        raw: INT;            (* a copy for the function *)
    END_VAR
    VAR_INPUT {ref}
        label: STRING;       (* the caller's text, not a copy *)
    END_VAR
    VAR_IN_OUT
        total: DINT;         (* read and written in the caller *)
    END_VAR
    VAR_OUTPUT
        valid: BOOL;         (* a second result *)
    END_VAR

    (* ... *)
END_FUNCTION
```

`VAR_INPUT` gives the function a copy, so a change inside the function stays inside it. This holds for large values as well: a string, an array, or a struct in `VAR_INPUT` is copied, and the caller does not see a write to it.

`VAR_INPUT {ref}` marks the whole block as "do not copy". The function then works on the value of the caller, which is what a foreign function usually expects, and what you want for a large value that the function only reads.

`VAR_IN_OUT` is for data that the function reads and writes. It is always the caller's variable.

`VAR_OUTPUT` carries a second result out of the call.


## Calling

A call gives the parameters their values by position, in the order of the declaration, or by name:

```iecst
r := Measure(raw, 'sensor', total, valid);
r := Measure(raw := raw, label := 'sensor', total := total, valid => valid);
```

`:=` gives a value to an input or to an in-out, `=>` takes a value out of an output. A call by position supplies **every** parameter, including the outputs. A missing argument is an error:

```
error[E032]: this POU takes 4 arguments but 2 arguments were supplied
```

An input with a default value (`factor: DINT := 2;`) is the exception: you can leave it out at the end of the list and get the default.

A call by name can leave out any input or output. An input that the call leaves out gets its default value, or the default value of its type (`0`, `FALSE`) when its declaration has none. An in-out is always necessary, because the function works on the variable of the caller. A call of a [function block](function-blocks.md) instance may leave out any parameter, because the instance keeps what the previous call gave it.


## Arrays of any size

A function that works on an array of any length declares the parameter with `*` instead of a range, one `*` per dimension. `LOWER_BOUND` and `UPPER_BOUND` then give the bounds of the array that the caller passed:

```iecst
FUNCTION Sum: DINT
    VAR_INPUT
        values: ARRAY[*] OF DINT;
    END_VAR
    VAR
        i: DINT;
    END_VAR

    FOR i := LOWER_BOUND(values, 1) TO UPPER_BOUND(values, 1) DO
        Sum := Sum + values[i];
    END_FOR
END_FUNCTION
```

The second argument of the two functions is the number of the dimension. They accept a parameter of this kind only, not an array of a fixed size. Such a parameter is always passed by reference, also in a `VAR_INPUT` block, and the compiler says so:

```
warning[E047]: Variable Length Arrays are always by-ref, even when declared in a by-value block
```


## Results that are not a number

A result type can be a string, an array, or a struct:

```iecst
FUNCTION Describe: STRING[20]
    VAR_INPUT
        code: DINT;
    END_VAR

    Describe := 'code ';
END_FUNCTION
```

The caller provides the memory for such a result. The compiler makes the result a hidden first parameter, a pointer to a variable of the caller, and the function writes the result through it.


## What's next

Functions forget. The next chapter is about the POUs that remember: [function blocks and programs](function-blocks.md).
