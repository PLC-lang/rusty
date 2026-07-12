# compound_sink

A sink whose target is a compound expression — an array element — rather than a
plain identifier. The validator admits any value expression as a sink target
(`results[1]`, `myStruct.field`, ...), so the transpiler must lower the target
by parsing it as an expression, exactly like it already does for sources; a
single flat identifier token `results[1]` resolves to nothing (E048).

```text
                    +----- function_0 -----+ (0)
   input  <-------->| inout     function_0 |--->  results[1]  (1)
                    +----------------------+

   <-->     an in-out pin (passed by reference)
   (0),(1)  evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test (the FBD network above), derived
  from an IDE export.
- `function_0.st` — the function it calls (`FUNCTION function_0 : DINT`, in-out
  `inout`).

The network means:

```text
__function_0_res_0 := function_0(inout := input);
results[1] := __function_0_res_0;
```
