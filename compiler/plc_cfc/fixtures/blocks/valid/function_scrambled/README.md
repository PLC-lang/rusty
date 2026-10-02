What: Scrambled priorities: sink `early` (1) reads the return pin *before* the
`myAdd` call (2); sink `late` (3) reads the output pin after. Statements stay in
raw priority order, so `early` reads the persisted temporary from the previous
cycle — the stateless analogue of a stateful block's prior-cycle read.

Illustrated:
```
        myAdd (2)
      +--------------------+
a --> | in1          myAdd | --> early (1)   [read before the call]
b --> | in2   myAddDoubled | --> late (3)
      +--------------------+
```
