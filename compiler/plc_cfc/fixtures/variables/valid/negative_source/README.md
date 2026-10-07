What: sources that negate a variable with a unary minus, bare and with
parentheses around the operand or the whole expression. Each one assigns the
negated value to its sink.

Illustrated:
```
-foo   --> a (1)
-(foo) --> b (2)
(-foo) --> c (3)
```
