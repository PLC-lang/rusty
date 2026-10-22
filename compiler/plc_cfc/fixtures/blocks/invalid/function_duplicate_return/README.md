What: The block carries two unnamed output pins. An empty `parameterName` marks
the return pin, and `myAdd` returns a single value, so the second unnamed pin
cannot be matched to anything; the block is rejected (E155).

Illustrated:
```
        myAdd (1)
      +-------------+
a --> | in1   myAdd |
b --> | in2   myAdd | --> result (2)
      +-------------+
      (both output pins are unnamed; the second is a duplicate return pin)
```
