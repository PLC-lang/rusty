What: execution control uses the external style, so nothing is injected into
the callee. A POU body assigning to ENO must fail to resolve, even while the
call site wires that POU's EN and ENO pins.

Illustrated:
```
              inst : sneaky (1)
            +-------------------+
trigger --> | EN            ENO | --> done (2)
            +-------------------+
            (body: ENO := FALSE;  -> E048)
```
