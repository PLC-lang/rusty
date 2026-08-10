What: negation bubbles on the variable side of block wires, and stacked with
pin bubbles on the same wire: the `in1` wire and the return-pin wire carry a
bubble on both ends (the NOTs nest), the `in2` and `myAddDoubled` wires carry
one on the variable side only. Verbatim IDE export.

Illustrated:
```
           +-------- myAdd ---------+
a1 o-----o-| in1              myAdd |-o-----o> b1 (1)
a2 o-------| in2       myAddDoubled |-------o> b2 (2)
           +------------------------+ (0)
```
