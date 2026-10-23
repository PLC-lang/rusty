What: calls hidden inside an expression: in an operand, in parentheses, in
the index of a sink, and in an array literal. A call is a block element of its
own, wherever it sits in the text, so each row is rejected with an
"unsupported CFC expression" error (E083).

Illustrated:
```
foo + MAX(foo, bar) --> a (1)                        (nested call, rejected)
(MAX(foo, bar))     --> b (2)                        (nested call, rejected)
foo                 --> values[MAX(foo, bar)] (3)    (call in index, rejected)
[MAX(foo, bar), 2]  --> values (4)                   (call in array literal, rejected)
```
