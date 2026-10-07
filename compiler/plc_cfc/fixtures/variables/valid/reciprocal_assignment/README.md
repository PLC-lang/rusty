What: Two assignments whose evaluation order matters — `bar := foo` (1) then
`foo := bar` (2). Confirms statements are emitted in `EvaluationPriority` order,
not document order. Modeled as a `PROGRAM`.

Illustrated:
```
foo --> bar (1)
bar --> foo (2)
```
