What: A source whose identifier is a function call, `MAX(foo, bar)`. A call is
a block element of its own, so this is rejected with an "unsupported CFC
expression" error (E083).

Illustrated:
```
MAX(foo, bar) --> result (1)    (call, rejected)
```
