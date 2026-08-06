What: a call to the builtin generic `SEL` selector. `main` checks both
selector positions: `cond = FALSE` yields the first input (`first = 10`),
`cond = TRUE` the second (`second = 20`).

Illustrated:
```
cond --> G   [SEL] SEL --> result (0, 1)
in0 ---> IN0
in1 ---> IN1
```
