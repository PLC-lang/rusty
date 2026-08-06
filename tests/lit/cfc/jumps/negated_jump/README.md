What: a jump guarded by a source whose negation bubble inverts the condition;
the jump fires when the guard is false. `main` checks a false guard skips the
assignment (`result = 0`) and a true guard runs it (`result = 42`).

Illustrated:
```
guard o--> JMP skipAssignment (0)
42 --> result (1)
LABEL skipAssignment (2)
```
