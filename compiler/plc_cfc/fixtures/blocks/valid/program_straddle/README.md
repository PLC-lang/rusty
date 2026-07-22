What: One block `counter` (1) whose single `out` pin feeds two sinks that straddle
its own priority — `before` (0) and `after` (2). The output is read once before the
call and once after, from the same persistent member: `before` sees last cycle's
value, `after` sees this cycle's. No temporary, no reordering.

Illustrated:
```
seed --> in [counter] out (1) --+--> before (0)
                                +--> after (2)
```
