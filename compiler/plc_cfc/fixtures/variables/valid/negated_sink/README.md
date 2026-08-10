What: a sink whose negation bubble inverts the incoming value before it is
stored, `bar := NOT foo`. The bubble sits on the sink's input pin; there is no
negated lvalue.

Illustrated:
```
foo --o bar (0)
```
