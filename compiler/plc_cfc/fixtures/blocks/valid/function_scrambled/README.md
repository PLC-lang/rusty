What: Scrambled priorities: sink `early` (0) reads the return pin *before* the
`myAdd` call (1); sink `late` (2) reads the output pin after. Statements stay in
raw priority order, so `early` reads the persisted temporary from the previous
cycle — the stateless analogue of a stateful block's prior-cycle read.

Illustrated:
```
        myAdd (1)
      +--------------------+
a --> | in1          myAdd | --> early (0)   [read before the call]
b --> | in2   myAddDoubled | --> late (2)
      +--------------------+
```
