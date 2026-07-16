What: A call to a **program-owned** action. `typeName="P.bump"` and there is no
`instanceName` — a program is a singleton, so the owner `P` *is* the reference.
The call is `P.bump(step := localIn)` and the output reads off the program,
`localOut := P.out`. Same split as the function-block case; the instance simply
falls back to the owner when no instance is declared.

Illustrated:
```
localIn --> step [P.bump] out (0) --> localOut (1)
```
