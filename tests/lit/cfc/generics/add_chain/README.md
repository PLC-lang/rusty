What: two builtin variadic `ADD` blocks in series; the intermediate temporary
resolves through generic inference. `main` checks the chained sum
(`(a + b) + c = 103000`).

Illustrated:
```
a --> [ADD] --> [ADD] --> result (0, 1, 2)
b -->      c -->
```
