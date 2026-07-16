What: The `program_chain` topology — `a` --> `b` --> `c` --> `result` — but with
priorities scrambled against the data flow: `c` (0), `result` (1), `a` (2), `b`
(3). Statements emit in raw priority order, so `c` is called before `a`/`b` even
exist this cycle; every block-to-block read is just a persisted member access, so
the scramble stays valid without reordering.

Illustrated:
```
seed --> in [a] out (2) --> in [b] out (3) --> in [c] out (0) --> result (1)
```
