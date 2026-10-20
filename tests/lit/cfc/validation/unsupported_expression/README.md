What: a source hides a function call in its index, and a jump takes a function
call as its condition. A call acts, so both are rejected (E083), even though
the project declares both functions.

Illustrated:
```
arr[f()]  --> r (1)           (call in an index, rejected)
isReady() --> JMP skip (2)    (call as a condition, rejected)
              LABEL skip (3)
```
