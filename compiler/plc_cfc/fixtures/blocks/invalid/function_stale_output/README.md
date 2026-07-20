What: The block still exposes an output pin `oldDoubled` that `myAdd` no longer
declares — a stale diagram after the callee's signature changed. The consumed
pin cannot be typed, so the block is rejected (E147).

Illustrated:
```
a --> in1 [myAdd] myAdd
b --> in2  (0)    oldDoubled? (stale) --> result (1)
```
