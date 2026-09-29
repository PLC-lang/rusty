# operator_functions

What: The builtins named by an operator keyword: the variadic `AND`, `OR` and
`XOR`, the two-input `MOD` and the one-input `NOT`. The keyword is no ST
expression on its own, the block's `typeName` still becomes the callee:
`AND(a, b, c)`, `MOD(IN1 := x, IN2 := y)`, `NOT(IN := a)`.

Illustrated:
```
a, b, c --> [AND] --> and_result (0, 5)
a, b    --> [OR]  --> or_result  (1, 6)
a, b    --> [XOR] --> xor_result (2, 7)
x, y    --> [MOD] --> mod_result (3, 8)
a       --> [NOT] --> not_result (4, 9)
```
