# evaluation_order

Two independent function calls. `myAdd` appears first in the document, but
`myMul` carries the lower evaluation priority — so the statements come out in
priority order, not document order.

```text
                     +----- myMul -----+ (1)
   localA  -------->| in1       myMul |--->  resultMul  (2)
   localB  -------->| in2             |
                     +-----------------+
                     +----- myAdd -----+ (3)
   localC  -------->| in1       myAdd |--->  resultAdd  (4)
   localD  -------->| in2             |
                     +-----------------+

   (1)-(4)  evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test (the FBD network above).
- `myMul.st` / `myAdd.st` — the two functions it calls.

Every block and sink is emitted in `priorityInNetwork` order, so even though
`myAdd` is listed first, the network means:

```text
__myMul_res_0 := myMul(in1 := localA, in2 := localB);
resultMul := __myMul_res_0;
__myAdd_res_1 := myAdd(in1 := localC, in2 := localD);
resultAdd := __myAdd_res_1;
```
