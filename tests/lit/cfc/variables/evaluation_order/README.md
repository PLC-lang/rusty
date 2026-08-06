What: three assignments to interleaved variables run purely in
`EvaluationPriority` order, not document order. `main` checks that `b`
captured the intermediate value of `a` before the second write (`a = 2,
b = 1`).

Illustrated:
```
1 --> a (0)
a --> b (1)
2 --> a (2)
```
