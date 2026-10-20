What: sources that hold an expression over literals and references: arithmetic,
a comparison, a `NOT` and an index expression. One expression carries a
negation bubble, which negates the whole expression. The last sink writes
through an index expression.

Illustrated:
```
a + 1          --> sum (1)
(a - b) * 2    --> scaled (2)
a > 5 AND flag --> ready (3)
NOT flag       --> idle (4)
a = b         o--> differs (5)
arr[i + 1]     --> item (6)
a              --> arr[i + 1] (7)
```
