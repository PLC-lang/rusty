What: sources that negate a variable with a unary minus, bare and with
parentheses around the operand or the whole expression. Each one assigns the
negated value to its sink.

Illustrated:
```
-foo   --> a (0)
-(foo) --> b (1)
(-foo) --> c (2)
```
