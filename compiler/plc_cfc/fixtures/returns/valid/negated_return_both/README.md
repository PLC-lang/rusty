What: a return with negation bubbles on both the source and the return element;
each inserts its own NOT, so they cancel at runtime,
`RETURN NOT NOT myCondition`.

Illustrated:
```
myCondition o--o RETURN (0)
```
