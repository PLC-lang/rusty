# generic_chain

What: Two chained calls of the generic `myGenAdd<T: ANY_NUM>` with mixed
widths: the first adds two INTs (`T = INT`), its captured output feeds the
second's `a` next to a DINT `c` (`T = join(INT, DINT) = DINT`). The second
temporary's type depends on the first's — the inference loop resolves
one link per round.

Illustrated:
```
        myGenAdd #1 (0)          myGenAdd #5 (1)
      +-----------------+      +-----------------+
a --> | a      myGenAdd | ---> | a      myGenAdd | --> result (2)
b --> | b               | c -> | b               |
      +-----------------+      +-----------------+
```
