# literal_input

A block input fed by a literal data source instead of a variable. A `DataSource`
identifier is parsed as an ST expression, so a constant like `100` flows straight
into the call.

```text
                     +----- myAdd -----+ (1)
   localA  --------->| in1       myAdd |--->  localResult  (2)
   100     --------->| in2             |
                     +-----------------+

   (1),(2)  evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test (the FBD network above).
- `myAdd.st` — the function it calls (`FUNCTION myAdd : DINT`, inputs `in1`, `in2`).

`localA` feeds `in1` and the literal `100` feeds `in2`, so the network means:

```text
temp_0 := myAdd(in1 := localA, in2 := 100);
localResult := temp_0;
```
