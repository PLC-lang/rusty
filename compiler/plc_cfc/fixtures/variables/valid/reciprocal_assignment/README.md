What: Two assignments whose evaluation order matters — `bar := foo` (0) then
`foo := bar` (1). Confirms statements are emitted in `EvaluationPriority` order,
not document order. Modeled as a `PROGRAM`.

Illustrated:
```
foo --> bar (0)
bar --> foo (1)
```
