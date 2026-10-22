What: an inout pin left unwired transpiles to an empty argument, which the
validation stage rejects; an inout must receive a real reference (E031). The
RUN line asserts the failure.

Illustrated:
```
        addInto (1)
      +---------------------+
5 --> | delta       addInto | --> result (2)
      | acc (unwired)       |
      +---------------------+
```
