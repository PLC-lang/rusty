What: negation bubbles on the source and the sink of a route through a
connector/continuation pair; each inserts its own NOT, so they cancel at
runtime, `bar := NOT NOT foo`.

Illustrated:
```
foo o--> x    x --o bar (0)
```
