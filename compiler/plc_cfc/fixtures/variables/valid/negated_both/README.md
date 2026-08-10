What: negation bubbles on both ends of one wire; each inserts its own NOT, so
they cancel at runtime, `bar := NOT NOT foo`. The transpiler keeps both to map
the diagram one-to-one.

Illustrated:
```
foo o--o bar (0)
```
