What: EN and ENO are not reserved words. The caller declares ordinary BOOL
variables named `EN` and `ENO` and wires them into the extension pins like any
other variable; the program compiles and behaves like a normal guarded call.

Illustrated:
```
              inst : counter (0)
            +-------------------+
     EN --> | EN            ENO | --> ENO (2)
localIn --> | in            out | --> localOut (1)
            +-------------------+
            (EN and ENO are ordinary caller variables)
```
