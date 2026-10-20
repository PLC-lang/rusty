What: sources that hold no single value without side effects. A call or an
assignment is rejected at any depth, also when it hides in an index, behind a
dereference or in an array literal; `**` is a call to `EXPT`. A list, a range,
`*`, an empty text and a hardware address are no value. Each one is rejected
with an "unsupported CFC expression" error (E083).

Illustrated:
```
arr[f()]     --> r (1)     (call in an index, rejected)
fp()^        --> r (2)     (call behind a dereference, rejected)
[f(), 1]     --> r (3)     (call in an array literal, rejected)
a ** 2       --> r (4)     (call to EXPT, rejected)
a := 1       --> r (5)     (assignment, rejected)
1 + (a := 1) --> r (6)     (nested assignment, rejected)
a, r         --> r (7)     (list, rejected)
1..5         --> r (8)     (range, rejected)
*            --> r (9)     (no value, rejected)
             --> r (10)    (empty, rejected)
%IX1.0       --> r (11)    (hardware address, rejected)
grid[f(), 1] --> r (12)    (call in one of several indices, rejected)
```
