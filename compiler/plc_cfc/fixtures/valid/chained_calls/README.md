# chained_calls

Two function calls in a chain: `myAdd`'s result feeds `myMul`. Because a block
output that feeds *another block* (rather than a sink) must still be evaluated
once, it is captured in a temporary that the next call reads.

```text
                     +----- myAdd -----+ (2)       +----- myMul -----+ (3)
   localA  --------->| in1       myAdd |---------->| IN1       myMul |------->  localResultA  (4)
   localB  --+------>| in2             |       +-->| IN2             |
             |       +-----------------+       |   +-----------------+
             +-------------------------------- +

   (2),(3),(4)  evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test (the FBD network above).
- `myAdd.st` / `myMul.st` — the two functions it calls.

`localA` and `localB` feed `myAdd`; its result and `localB` feed `myMul`; and
`myMul`'s result is written to `localResultA`, so the network means:

```text
__myAdd_res_0 := myAdd(in1 := localA, in2 := localB);
__myMul_res_1 := myMul(IN1 := __myAdd_res_0, IN2 := localB);
localResultA := __myMul_res_1;
```
