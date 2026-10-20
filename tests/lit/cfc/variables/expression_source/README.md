What: sources that hold expressions over literals and references feed sinks
directly, and two sinks write through an index expression, one of them into a
multidimensional array, which the last source reads back. The bubble on
`a = b` negates the whole comparison, not only `a`. `main` runs one cycle and
checks every result (`a = 3`, `b = 4`, `i = 1`, `flag = TRUE`).

Illustrated:
```
a + 1          --> sum (1)
(a - b) * 2    --> scaled (2)
a > 2 AND flag --> ready (3)
NOT flag       --> idle (4)
a = b         o--> differs (5)
arr[i + 1]     --> item (6)
a              --> arr[i + 1] (7)
a              --> grid[i, i + 1] (8)
grid[1, 2] * 2 --> cell (9)
```
