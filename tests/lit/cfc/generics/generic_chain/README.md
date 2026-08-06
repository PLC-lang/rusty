What: two user-defined generic `myGenAdd<T: ANY_NUM>` calls in series with
mixed input widths (INT inputs into the first link, DINT into the second);
each link resolves its own binding. `main` checks the chained sum
(`result = 103000`).

Illustrated:
```
a --> [myGenAdd] --> [myGenAdd] --> result (0, 1, 2)
b -->            c -->
```
