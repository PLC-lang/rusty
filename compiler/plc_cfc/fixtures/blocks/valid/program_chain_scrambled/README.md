What: The `program_chain` topology — `a` --> `b` --> `c` --> `result` — but with
priorities scrambled against the data flow: `c` (1), `result` (2), `a` (3), `b`
(4). Statements emit in raw priority order, so `c` is called before `a`/`b` even
exist this cycle; every block-to-block read is just a persisted member access, so
the scramble stays valid without reordering.

Illustrated:
```
           a (3)            b (4)            c (1)
         +----------+     +----------+     +----------+
seed --> | in   out | --> | in   out | --> | in   out | --> result (2)
         +----------+     +----------+     +----------+
```
