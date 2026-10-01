What: calls to the builtins named by an operator keyword: the variadic `AND`,
`OR` and `XOR`, the two-input `MOD` and the one-input `NOT`. `main` runs the
diagram twice and checks each result against its ST operator.

Illustrated:
```
a, b, c --> [AND] --> and_result (0, 5)
a, b    --> [OR]  --> or_result  (1, 6)
a, b    --> [XOR] --> xor_result (2, 7)
x, y    --> [MOD] --> mod_result (3, 8)
a       --> [NOT] --> not_result (4, 9)
```
