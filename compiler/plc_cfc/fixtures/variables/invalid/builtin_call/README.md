What: sources that call the built-in `SEL`, bare and nested in arithmetic. A
call is a block element of its own, also the call of a built-in, so each row is
rejected with an "unsupported CFC expression" error (E083).

Illustrated:
```
SEL(flag, foo, bar)   --> a (1)    (built-in call, rejected)
foo + SEL(flag, 1, 2) --> b (2)    (nested built-in call, rejected)
```
