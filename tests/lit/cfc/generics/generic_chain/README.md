What: two user-defined generic `myGenAdd<T: ANY_NUM>` calls in series with
mixed input widths (INT inputs into the first link, DINT into the second);
each link resolves its own binding. `main` checks the chained sum
(`result = 103000`).

Illustrated:
```
        myGenAdd (1)              myGenAdd (2)
      +-----------------+       +-----------------+
a --> | a      myGenAdd | ----> | a      myGenAdd | --> result (3)
b --> | b               | c --> | b               |
      +-----------------+       +-----------------+
```
