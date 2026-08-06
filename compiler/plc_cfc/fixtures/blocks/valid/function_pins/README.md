What: negation bubbles on the pins of a function block element only (parameter
side): the `IN1` input pin, the return pin, and the declared output pin. All
sources and sinks stay plain. Verbatim IDE export; note the IDE wrote the input
pins as `IN1`/`IN2` although `myAdd` declares `in1`/`in2` (matching is
case-insensitive).

Illustrated:
```
           +-------- myAdd ---------+
a1 ------o-| IN1              myAdd |-o------> b1 (2)
a2 --------| IN2       myAddDoubled |-o------> b2 (1)
           +------------------------+ (0)
```
