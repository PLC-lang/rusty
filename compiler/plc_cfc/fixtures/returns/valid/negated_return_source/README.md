What: a return guarded by a source whose negation bubble inverts the condition
it feeds, `RETURN NOT myCondition`. The bubble sits on the source, not on the
return element (the IDE still writes an explicit `inNegated="false"` there).

Illustrated:
```
myCondition o-- RETURN (0)
```
