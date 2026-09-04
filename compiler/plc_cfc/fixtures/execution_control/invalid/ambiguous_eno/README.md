What: a block with execution control whose callee declares an own `ENO`
parameter, so two pins named `ENO` exist. The export format does not yet
distinguish the control pin from the parameter, so any choice is a silent
guess; the compiler panics until the format defines the disambiguation.

Illustrated:
```
              inst : counter (0)
            +-------------------+
trigger --> | EN            ENO | --> done (2)
localIn --> | in            out | --> localOut (1)
            |               ENO | --> ?
            +-------------------+
            (two ENO pins: the guard mirror and the callee's own parameter)
```
