What: One source feeding two sinks — `bar := foo` (1) and `baz := foo` (2). A
single `ConnectionPointOut` is referenced by multiple `Connection`s. Modeled as a
`FUNCTION_BLOCK`.

Illustrated:
```
foo --+--> bar (1)
      +--> baz (2)
```
