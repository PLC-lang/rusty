What: one connector consumed by two continuations with the same label; the
routed value fans out into two sinks. `main` checks both received it
(`bar = 42, baz = 42`).

Illustrated:
```
foo --> x    x --> bar (0)
             x --> baz (1)
```
