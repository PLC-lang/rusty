What: EN wired from a stateless function's return value, the classic
division-by-zero guard. The guard expression reads the `__out` capture, so the
EN wire alone must force the temporary into existence.

Illustrated:
```
      isNonZero (1)              safeDiv (2)
    +--------------+          +------------------+
b ->| val isNonZero| --EN --> | EN               |
    +--------------+     a -> | dividend safeDiv | --> result (3)
                         b -> | divisor          |
                              +------------------+
```
