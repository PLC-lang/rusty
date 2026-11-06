What: sources that hold expressions instead of plain names. `sa * 2 + 1` and
`sb * 10` feed the inputs of two function block instances, and `outA + outB`
feeds a sink. `main` runs two cycles and checks the results (`a = 2 * 3 = 6`,
`b = 2 * 10 = 20`, `sum = 6 + 20 = 26`).

Illustrated:
```
                 a : counter (1)
               +--------------+
sa * 2 + 1 --> | step   total | --> outA (2)
               +--------------+

                 b : counter (3)
               +--------------+
sb * 10    --> | step   total | --> outB (4)
               +--------------+

outA + outB --> sum (5)
```
