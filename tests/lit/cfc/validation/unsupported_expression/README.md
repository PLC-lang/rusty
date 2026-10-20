What: a source hides a function call in its index, and a jump takes a function
call as its condition. A call acts, so both are rejected (E083), even though
the project declares both functions. A source with a stray operator is
rejected as well, because the parser can only read it by skipping the operator.

Illustrated:
```
arr[f()]  --> r (1)           (call in an index, rejected)
r + / 2   --> r (2)           (stray operator, rejected)
isReady() --> JMP skip (3)    (call as a condition, rejected)
              LABEL skip (4)
```
