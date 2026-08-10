What: an inout pin left unwired transpiles to an empty argument, which the
validation stage rejects — an inout must receive a real reference (E031). The
RUN line asserts the failure.

Illustrated:
```
5 --> delta [addInto] addInto --> result (0, 1)
      acc (unwired)
```
