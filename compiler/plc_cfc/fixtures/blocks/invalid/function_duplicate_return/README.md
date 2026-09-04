What: The block carries two unnamed output pins. An empty `parameterName` marks
the return pin, and `myAdd` returns a single value, so the second unnamed pin
cannot be matched to anything; the block is rejected (E152).

Illustrated:
```
a --> in1 [myAdd] (return)
b --> in2  (0)    (return)? (duplicate) --> result (1)
```
