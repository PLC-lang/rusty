What: A block `program_0` (0) with an inversion bubble on every pin. A negated
input/in_out wraps its wired value in `NOT`; a negated output wraps each read.
`NOT` on a numeric is bitwise, so `in`/`out` are valid — but a negated in_out
yields `NOT localInOut`, which the main pipeline rejects (E031, an in_out
argument must be a reference). This fixture is therefore transpile-only.

Illustrated:
```
localIn    --> in    (¬) [program_0] out (¬) (0) --> localOut (1)
localInOut --> inout (¬)
```
