What: EN and ENO are not reserved words. The caller declares ordinary BOOL
variables named `EN` and `ENO` and wires them into the EN/ENO pins like any
other variable; the program compiles and behaves like a normal guarded call.

Illustrated:
```
              inst : counter (1)
            +-------------------+
     EN --> | EN            ENO | --> ENO (3)
localIn --> | in            out | --> localOut (2)
            +-------------------+
            (EN and ENO are ordinary caller variables)
```
