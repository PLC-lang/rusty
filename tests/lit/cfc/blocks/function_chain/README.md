What: two `myAdd` calls in series; the first return value feeds the second
call's input, captured through a temporary. `main` checks the chained sum
(`(seed + k) + k = 11` for `seed = 3, k = 4`).

Illustrated:
```
seed --> in1 [myAdd] --> in1 [myAdd] --> result (0, 1, 2)
k ------> in2         k --> in2
```
