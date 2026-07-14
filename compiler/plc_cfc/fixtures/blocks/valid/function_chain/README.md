What: Two `myAdd` calls chained — the first's return feeds the second's `in1`,
whose return feeds `result`; `k` feeds the `in2` of both. Each call gets its own
temporary (keyed by the block's globalId), so two invocations of the same
stateless function stay distinct.

Illustrated:
```
          myAdd #1 (0)                myAdd #10 (1)
        +--------------------+      +--------------------+
seed -->| in1          myAdd |----->| in1          myAdd |--> result (2)
   k -->| in2   myAddDoubled |  k -->| in2   myAddDoubled |
        +--------------------+      +--------------------+
```
