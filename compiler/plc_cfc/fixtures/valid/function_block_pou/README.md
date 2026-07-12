# function_block_pou

The transpiled POU is a **FUNCTION_BLOCK**. Its body is an FBD network, and each
`VAR_OUTPUT` is written by a `DataSink` named after the output variable.

```text
              +----- myAdd -----+ (1)
   a  ------->| in1       myAdd |--->  sum  (2)
   b  ------->| in2             |
              +-----------------+

   sum      a VAR_OUTPUT of the function block (a sink named after the output)
   (1),(2)  evaluation-priority badges shown by the IDE
```

- `myFb.cfc` — the FUNCTION_BLOCK under test (`FUNCTION_BLOCK myFb`).
- `myAdd.st` — the function it calls.

The inputs `a`/`b` feed `myAdd`, whose result is written to the output `sum`, so
the POU means:

```text
FUNCTION_BLOCK myFb
VAR_INPUT
    a : DINT;
    b : DINT;
END_VAR
VAR_OUTPUT
    sum : DINT;
END_VAR
    __myAdd_res_0 := myAdd(in1 := a, in2 := b);
    sum := __myAdd_res_0;
END_FUNCTION_BLOCK
```
