What: a negated source routed through a connector/continuation pair; the
bubble travels with the traced source, `bar := NOT foo`. Connectors and
continuations themselves can not be negated, only the nodes wired to them.

Illustrated:
```
foo o--> x    x --> bar (0)
```
