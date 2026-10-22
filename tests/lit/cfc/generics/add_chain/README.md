What: two builtin variadic `ADD` blocks in series; the intermediate temporary
resolves through generic inference. `main` checks the chained sum
(`(a + b) + c = 103000`).

Illustrated:
```
        ADD (1)             ADD (2)
      +-----------+       +-----------+
a --> | IN1   ADD | ----> | IN1   ADD | --> result (3)
b --> |           | c --> |           |
      +-----------+       +-----------+
```
