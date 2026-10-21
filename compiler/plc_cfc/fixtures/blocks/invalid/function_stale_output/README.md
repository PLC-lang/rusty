What: The block still exposes an output pin `oldDoubled` that `myAdd` no longer
declares — a stale diagram after the callee's signature changed. The consumed
pin cannot be typed, so the block is rejected (E147).

Illustrated:
```
        myAdd (1)
      +------------------+
a --> | in1        myAdd |
b --> | in2   oldDoubled | --> result (2)
      +------------------+
      (stale: myAdd does not declare oldDoubled)
```
