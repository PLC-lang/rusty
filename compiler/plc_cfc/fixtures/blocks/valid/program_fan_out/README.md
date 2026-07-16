What: One program block `counter` (0) whose single `out` pin feeds two sinks,
`a` (1) and `b` (2). A persistent output can be read any number of times, so each
sink is an independent member access — no temporary is introduced.

Illustrated:
```
seed --> in [counter] out (0) --+--> a (1)
                                +--> b (2)
```
