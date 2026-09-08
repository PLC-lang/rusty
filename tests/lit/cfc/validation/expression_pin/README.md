What: a data source holding an arithmetic expression (`a1 + 1`) feeds the first
input pin of a `myAdd` block. A variable element may only hold a literal or a
reference, so the resolver rejects it (E083). The source has no execution order
of its own, so the diagnostic is located at the consuming block plus the pin it
enters through: `expression_pin.cfc.expression_pin:0:0`.

Illustrated:
```
a1 + 1 --> IN1 [myAdd] myAdd        --> b1 (2)
a2     --> IN2         myAddDoubled --> b2 (1)
                  (0)
```
