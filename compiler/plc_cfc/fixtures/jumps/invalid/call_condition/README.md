What: a jump whose condition is a function call. A condition follows the same
rule as any other source, so the call is rejected with an "unsupported CFC
expression" error (E083).

Illustrated:
```
isReady() --> JMP skip (1)    (call, rejected)
              LABEL skip (2)
```
