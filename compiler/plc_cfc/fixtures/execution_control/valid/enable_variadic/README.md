What: a variadic builtin call behind an EN guard. Variadic arguments pass by
pin order without parameter names; the guard is orthogonal and wraps the
captured call like any other.

Illustrated:
```
              ADD (1)
            +----------+
trigger --> | EN       |
      x --> |      ADD | --> r (2)
      y --> |          |
            +----------+
```
