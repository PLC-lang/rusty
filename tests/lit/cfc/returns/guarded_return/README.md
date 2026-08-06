What: a conditional early return ahead of an assignment. `main` checks a false
guard falls through and the assignment runs (`result = 42`), while a true
guard returns early and `result` keeps its prior value (`result = 7`).

Illustrated:
```
guard --> RETURN (0)
42 --> result (1)
```
