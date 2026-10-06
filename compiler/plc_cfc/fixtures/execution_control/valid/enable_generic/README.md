What: a generic function behind an EN guard. The `__out` temporary starts with
a generic placeholder type, and the infer fixed point must find the capture
inside the guard's IF body to resolve it; a harvest that only walks top-level
statements never sees it.

Illustrated:
```
              myGenAdd<T: ANY_NUM> (1)
            +------------------------+
trigger --> | EN                     |
      x --> | a             myGenAdd | --> result (2)
      y --> | b                      |
            +------------------------+
```
