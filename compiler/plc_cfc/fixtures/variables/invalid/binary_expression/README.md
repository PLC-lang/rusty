What: A source whose identifier is an arithmetic expression, `foo + 1`. A source
or sink may only carry a variable reference or a literal, not an expression, so
this is rejected with an "unsupported CFC expression" error (E083).

Illustrated:
```
foo + 1 --> bar (0)    (expression — rejected)
```
