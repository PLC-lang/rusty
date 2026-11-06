What: a data source holding a function call (`myAdd(a1, 1)`) feeds the first
input pin of a `myAdd` block. A call is a block element of its own, so the
resolver rejects it (E083). The source has no execution order of its own, so the
diagnostic is located at the consuming block plus the pin it enters through:
`expression_pin.cfc.expression_pin:1:0`.

Illustrated:
```
                   myAdd (1)
                 +--------------------+
myAdd(a1, 1) --o | IN1          myAdd | o--> b1 (3)
          a2 --> | IN2   myAddDoubled | o--> b2 (2)
                 +--------------------+
                 (o = negated pin; the call source myAdd(a1, 1) is rejected)
```
