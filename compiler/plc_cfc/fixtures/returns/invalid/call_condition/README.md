What: a return whose condition is a function call. A condition follows the
same rule as any other source, so the call is rejected with an "unsupported CFC
expression" error (E083).

Illustrated:
```
isReady() --> RETURN (1)    (call, rejected)
```
