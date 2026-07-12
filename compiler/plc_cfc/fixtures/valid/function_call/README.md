# function_call

The simplest CFC program: a single function call whose result is written to a
variable.

```text
                     +-------- myAdd --------+  (1)
   localA  --------->| in1              myAdd|--------->  localResult  (2)
   localB  --------->| in2                   |
                     +-----------------------+

   (1),(2)  evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test (the FBD network above).
- `myAdd.st` — the function it calls (`FUNCTION myAdd : DINT`, inputs `in1`, `in2`).

`localA` and `localB` feed the function's `in1` / `in2` inputs. The block's
output pin is named `myAdd` — the same as the type — which is how a function's
return value is encoded. The block is evaluated once into a temporary, which the
`localResult` sink then reads, so the network means:

```text
__myAdd_res_0 := myAdd(in1 := localA, in2 := localB);
localResult := __myAdd_res_0;
```
