# function_pou

The transpiled POU is a **FUNCTION** (the existing fixtures are all `PROGRAM`s).
Its body is an FBD network, and its return value is set by assigning the
function's own name — modelled as a `DataSink` whose identifier equals the
function name.

```text
              +----- myAdd -----+ (1)
   a  ------->| in1       myAdd |--->  myFunc  (2)
   b  ------->| in2             |
              +-----------------+

   myFunc   the FUNCTION's return value (a sink named after the function)
   (1),(2)  evaluation-priority badges shown by the IDE
```

- `myFunc.cfc` — the FUNCTION under test (`FUNCTION myFunc : DINT`).
- `myAdd.st` — the function it calls.

The inputs `a`/`b` feed `myAdd`, whose result becomes the function's return value
`myFunc`, so the POU means:

```text
FUNCTION myFunc : DINT
VAR_INPUT
    a : DINT;
    b : DINT;
END_VAR
    temp_0 := myAdd(in1 := a, in2 := b);
    myFunc := temp_0;
END_FUNCTION
```
