What: a conditional jump over an assignment. `main` checks both branches: a
false guard falls through and the assignment runs (`y = 42`); a true guard
takes the jump and `y` keeps its prior value (`y = 7`).

Illustrated:
```
cond --> JMP skip (0)
x --> y (1)
LABEL skip (2)
```
