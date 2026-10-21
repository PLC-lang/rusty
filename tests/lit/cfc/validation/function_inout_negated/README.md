What: negation bubbles on an inout wire (source side and pin side). The IDE
permits drawing them and the resolver transpiles them like any input pin, but
an inout passes by reference and a NOT expression has no storage behind it, so
the validation stage rejects the build (E031). The RUN line asserts the
failure.

Illustrated:
```
              addInto (1)
            +---------------------+
  a1 -----> | delta       addInto | --> b1 (2)
acc1 o<-->o | acc                 |
            +---------------------+
            (o = negated pin; acc1 is also negated on the source side)
```
