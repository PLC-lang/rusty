What: two instances of the same function block called in one network; each
keeps its own state. `main` runs two cycles and checks the accumulators stayed
separate (`a = 2, b = 20` from `sa = 1`, `sb = 10`).

Illustrated:
```
sa --> step [a : counter] total --> outA (0, 1)
sb --> step [b : counter] total --> outB (2, 3)
```
