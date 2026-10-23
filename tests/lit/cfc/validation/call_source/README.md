What: sources that call functions: `MAX` from the standard library, the same
call inside the condition of a return, and the built-in `SEL`. A call is a
block element of its own, so each row is rejected (E083).

Illustrated:
```
MAX(a, b)        --> result (1)    (call, rejected)
a > MAX(a, b)    --> RETURN (2)    (call, rejected)
SEL(a > b, a, b) --> result (3)    (built-in call, rejected)
```
