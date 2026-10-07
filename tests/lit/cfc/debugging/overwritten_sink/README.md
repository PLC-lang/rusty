What: the debugger stops of a network whose second sink overwrites the first.
`x1` and `x2` are locals with start values, so the backend computes ADD and SUB
ahead of time, and `x2` (order 2) is overwritten by `x2` (order 4) before
anything reads it. Each element must still keep its own row in the line table
of `overwritten_sink.cfc.overwritten_sink`, at line = execution order.

Illustrated:
```
            +---- ADD (1) ----+
 -x1 ------>| IN1             |
  x2 ------>|                 |----> x2 (2)
            +-----------------+

            +---- SUB (3) ----+
  x1 ------>| IN1             |
  x2 ------>| IN2             |----> x2 (4)
            +-----------------+
```
