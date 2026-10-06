What: a block with execution control whose callee declares an own `EN`
parameter, so two pins named `EN` exist. The export format does not yet
distinguish the control pin from the parameter, so any choice is a silent
guess; the compiler panics until the format defines the disambiguation.

Illustrated:
```
              inst : counter (1)
            +-------------------+
trigger --> | EN            ENO | --> done (3)
        ?-- | EN                |
localIn --> | in            out | --> localOut (2)
            +-------------------+
            (two EN pins: the guard and the callee's own parameter)
```
