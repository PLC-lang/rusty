What: One block `counter` (2) whose single `out` pin feeds two sinks that straddle
its own priority — `before` (1) and `after` (3). The output is read once before the
call and once after, from the same persistent member: `before` sees last cycle's
value, `after` sees this cycle's. No temporary, no reordering.

Illustrated:
```
           counter (2)
         +------------+
seed --> | in     out | --+--> before (1)
         +------------+   +--> after (3)
```
