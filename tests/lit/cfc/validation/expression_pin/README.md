What: a data source holding an arithmetic expression (`a1 + 1`) feeds the first
input pin of a `myAdd` block. A variable element may only hold a literal or a
reference, so the resolver rejects it (E083). The source has no execution order
of its own, so the diagnostic is located at the consuming block plus the pin it
enters through: `expression_pin.cfc.expression_pin:1:0`.

Illustrated:
```
             myAdd (1)
           +--------------------+
a1 + 1 --o | IN1          myAdd | o--> b1 (3)
    a2 --> | IN2   myAddDoubled | o--> b2 (2)
           +--------------------+
           (o = negated pin; the expression source a1 + 1 is rejected)
```
