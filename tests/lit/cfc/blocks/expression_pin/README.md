What: the inputs of two function block instances are fed by arithmetic
expressions. Each expression is evaluated when its block is called. `main`
runs two cycles and checks the accumulators (`a = 4, b = 44` from `sa = 1`,
`sb = 10`).

Illustrated:
```
                    a : counter (1)
                  +--------------+
       sa + 1 --> | step   total | --> outA (2)
                  +--------------+

                    b : counter (3)
                  +--------------+
(sa + sb) * 2 --> | step   total | --> outB (4)
                  +--------------+
```
