# shared_result

A single function call whose result is consumed by **two** sinks. A block is
evaluated once, so the shared result is captured in a temporary that both sinks
read — rather than calling `myAdd` twice.

```text
                     +-------- myAdd --------+  (1)
   localA  --------->| in1              myAdd|-------+-------->  localResultA  (2)
   localB  --------->| in2                   |       |
                     +-----------------------+       +-------->  localResultB  (3)

   (1),(2),(3)  evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test (the FBD network above).
- `myAdd.st` — the function it calls (`FUNCTION myAdd : DINT`, inputs `in1`, `in2`).

`localA` and `localB` feed `myAdd`; its result fans out to both `localResultA`
and `localResultB`, so the network means:

```text
__myAdd_res_0 := myAdd(in1 := localA, in2 := localB);
localResultA := __myAdd_res_0;
localResultB := __myAdd_res_0;
```
