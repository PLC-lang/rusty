What: a value routed through three connector/continuation pairs in series;
the trace walks the whole chain back to the source. `main` checks the value
survived the routing (`bar = 7`).

Illustrated:
```
foo --> a    a --> b    b --> c    c --> bar (0)
```
