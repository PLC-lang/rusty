What: a jump guarded by a source whose negation bubble inverts the condition it
feeds (the jump fires when the condition is false). The bubble sits on the
source, not on the jump element (the IDE still writes an explicit
`inNegated="false"` there).

Illustrated:
```
myCondition o-- JMP skipAssignment (0)

x --> y (1)

LABEL skipAssignment (2)
```
