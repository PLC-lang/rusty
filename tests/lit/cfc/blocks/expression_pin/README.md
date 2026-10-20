What: the inputs of two function block instances are fed by arithmetic
expressions. Each expression is evaluated when its block is called. `main`
runs two cycles and checks the accumulators (`a = 4, b = 44` from `sa = 1`,
`sb = 10`).

Illustrated:
```
sa + 1        --> step [a : counter] total --> outA (0, 1)
(sa + sb) * 2 --> step [b : counter] total --> outB (2, 3)
```
