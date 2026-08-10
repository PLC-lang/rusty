What: a jump with negation bubbles on both the source and the jump element;
each inserts its own NOT, so they cancel at runtime and the jump fires when the
condition is true, `JMP skipAssignment IF NOT NOT myCondition`.

Illustrated:
```
myCondition o--o JMP skipAssignment (0)

x --> y (1)

LABEL skipAssignment (2)
```
