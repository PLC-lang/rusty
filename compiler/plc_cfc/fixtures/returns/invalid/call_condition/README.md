What: a return whose condition source calls a function, `MAX(foo, bar) > 5`.
A condition source follows the same rules as any other source, so the call is
rejected with an "unsupported CFC expression" error (E083).

Illustrated:
```
MAX(foo, bar) > 5 --> RETURN (1)    (call, rejected)
```
