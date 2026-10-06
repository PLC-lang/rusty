What: The `program_chain` topology — `a` --> `b` --> `c` --> `result` — but with
priorities scrambled against the data flow: `c` (1), `result` (2), `a` (3), `b`
(4). Statements emit in raw priority order, so `c` is called before `a`/`b` even
exist this cycle; every block-to-block read is just a persisted member access, so
the scramble stays valid without reordering.

Illustrated:
```
seed --> in [a] out (3) --> in [b] out (4) --> in [c] out (1) --> result (2)
```
