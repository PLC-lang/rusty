What: the classic EN use case, a division-by-zero guard. A comparison function
computes the enable, so the guarded division only runs while the divisor is
nonzero; a zero divisor skips the call and `result` holds its last value
instead of faulting the runtime.

Illustrated:
```
      isNonZero (1)              safeDiv (2)
    +--------------+          +------------------+
b ->| val isNonZero| --EN --> | EN               |
    +--------------+     a -> | dividend safeDiv | --> result (3)
                         b -> | divisor          |
                              +------------------+
```
