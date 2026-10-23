What: identifiers that are not exactly one expression: a syntax error, text
after the expression, an assignment, a list, a range and a multiplied element.
A range and a multiplied element are only parts of an array literal. Each row
is rejected with an "unsupported CFC expression" error (E083) instead of being
cut short.

Illustrated:
```
foo +    --> a (1)      (syntax error, rejected)
foo      --> a b (2)    (text after the expression, rejected)
foo := 1 --> a (3)      (assignment, rejected)
foo, bar --> a (4)      (list, rejected)
1..5     --> a (5)      (range, rejected)
3(foo)   --> a (6)      (multiplied element, rejected)
```
