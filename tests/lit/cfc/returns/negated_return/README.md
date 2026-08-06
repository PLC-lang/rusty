What: a return whose negation bubble inverts its wired condition; the early
return fires when the guard is false. `main` checks a false guard skips the
assignment (`result = 0`) and a true guard runs it (`result = 42`).

Illustrated:
```
guard --o RETURN (0)
42 --> result (1)
```
