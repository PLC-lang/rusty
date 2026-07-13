What: A source whose identifier is a function call, `MAX(foo, bar)`. Sources and
sinks may only carry a variable reference or a literal, so this is rejected with
an "unsupported CFC expression" error (E083).

Illustrated:
```
MAX(foo, bar) --> result (0)    (call — rejected)
```
