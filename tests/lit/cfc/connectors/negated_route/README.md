What: negation bubbles on the endpoints of connector routes — a source bubble
ahead of one connector (`b := NOT a`) and a sink bubble behind another
continuation (`d := NOT c`). Connectors and continuations themselves can not
be negated. `main` checks both directions with flipped inputs.

Illustrated:
```
a o--> route1    route1 --> b (0)
c ---> route2    route2 --o d (1)
```
