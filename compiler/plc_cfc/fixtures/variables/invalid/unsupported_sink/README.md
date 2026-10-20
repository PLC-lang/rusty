What: sinks that cannot be written to. A literal or an expression is no
assignment target, a call in an index is rejected like in a source, and a sink
in parentheses is no target either. Each one is rejected with an "unsupported
CFC expression" error (E083).

Illustrated:
```
a --> 5            (1)    (literal, rejected)
a --> r + 1        (2)    (expression, rejected)
a --> arr[f()]     (3)    (call in an index, rejected)
a --> (r)          (4)    (parentheses, rejected)
a --> grid[f(), 1] (5)    (call in one of several indices, rejected)
```
