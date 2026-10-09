What: `program_straddle` for a function-block instance — one instance `inst` (2)
whose `out` feeds two sinks that straddle its priority, `before` (1) and `after`
(3). Priority ordering is shared with programs, so this only re-confirms it holds
when the reference is an instance: `before` sees last cycle's value, `after` this
cycle's, both off the same persistent member; no reordering, no temporary.

Illustrated:
```
seed --> in [inst : counter] out (2) --+--> before (1)
                                       +--> after (3)
```
