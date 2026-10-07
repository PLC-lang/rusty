What: execution control with only the EN side used. The ENO pin is absent from
the declaration, which is legal: no consumer needs the value. Only the guard is
generated.

Illustrated:
```
              inst : counter (1)
            +-------------------+
trigger --> | EN                |
localIn --> | in            out | --> localOut (2)
            +-------------------+
```
