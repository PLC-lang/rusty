What: One program block `counter` (1) whose single `out` pin feeds two sinks,
`a` (2) and `b` (3). A persistent output can be read any number of times, so each
sink is an independent member access — no temporary is introduced.

Illustrated:
```
seed --> in [counter] out (1) --+--> a (2)
                                +--> b (3)
```
