What: One source feeding two sinks — `bar := foo` (0) and `baz := foo` (1). A
single `ConnectionPointOut` is referenced by multiple `Connection`s. Modeled as a
`FUNCTION_BLOCK`.

Illustrated:
```
foo --+--> bar (0)
      +--> baz (1)
```
