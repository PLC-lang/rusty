What: sources that hold no single value without side effects. A call or an
assignment is rejected at any depth, also when it hides in an index, behind a
dereference, in an array literal or in a repetition such as `3(f())`; `**` is
a call to `EXPT`. A list, a range, `*`, an empty text and a hardware address
are no value. Each one is rejected with an "unsupported CFC expression" error
(E083). A repetition is checked once, so even a huge count is rejected without
expanding it. Text that the parser cannot read in full is rejected too, because
its error recovery would turn it into a different expression.

Illustrated:
```
arr[f()]              --> r (1)     (call in an index, rejected)
fp()^                 --> r (2)     (call behind a dereference, rejected)
[f(), 1]              --> r (3)     (call in an array literal, rejected)
a ** 2                --> r (4)     (call to EXPT, rejected)
a := 1                --> r (5)     (assignment, rejected)
1 + (a := 1)          --> r (6)     (nested assignment, rejected)
a, r                  --> r (7)     (list, rejected)
1..5                  --> r (8)     (range, rejected)
*                     --> r (9)     (no value, rejected)
                      --> r (10)    (empty, rejected)
%IX1.0                --> r (11)    (hardware address, rejected)
grid[f(), 1]          --> r (12)    (call in one of several indices, rejected)
1 + [4294967295(f())] --> r (13)    (call in a repetition, rejected)
a + / r               --> r (14)    (stray operator, rejected)
a garbage             --> r (15)    (trailing text, rejected)
```
