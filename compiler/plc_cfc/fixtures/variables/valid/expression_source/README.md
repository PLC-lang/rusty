What: sources that hold an expression over literals and references: arithmetic,
a comparison, a `NOT` and an index expression. One expression carries a
negation bubble, which negates the whole expression. Two sinks write
through an index expression, one of them into a multidimensional array, which
the last source reads back.

Illustrated:
```
a + 1          --> sum (1)
(a - b) * 2    --> scaled (2)
a > 5 AND flag --> ready (3)
NOT flag       --> idle (4)
a = b         o--> differs (5)
arr[i + 1]     --> item (6)
a              --> arr[i + 1] (7)
a              --> grid[i, i + 1] (8)
grid[1, 2] * 2 --> cell (9)
```
