What: a function-block call with execution control. `trigger` wires into the
block's EN pin, so the call runs only while `trigger` is TRUE; the ENO pin
mirrors EN and feeds the `done` sink. ENO resolves transparently to the EN
source, so no extra storage is generated: `done := trigger`.

Illustrated:
```
              inst : counter (0)
            +-------------------+
trigger --> | EN            ENO | --> done (2)
localIn --> | in            out | --> localOut (1)
            +-------------------+
```
