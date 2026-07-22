What: `program_straddle` for a function-block instance — one instance `inst` (1)
whose `out` feeds two sinks that straddle its priority, `before` (0) and `after`
(2). Priority ordering is shared with programs, so this only re-confirms it holds
when the reference is an instance: `before` sees last cycle's value, `after` this
cycle's, both off the same persistent member; no reordering, no temporary.

Illustrated:
```
seed --> in [inst : counter] out (1) --+--> before (0)
                                       +--> after (2)
```
