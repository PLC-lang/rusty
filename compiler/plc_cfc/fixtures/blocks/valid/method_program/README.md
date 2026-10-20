What: A call to a **program-owned** method. `typeName="P.bump"` and there is no
`instanceName`; the owner `P` is the reference, so the call is
`P.bump(step := localIn)`. The return value is captured into a temporary during
the call, like a function's, and `localOut` reads that temporary.

Illustrated:
```
localIn --> step [P.bump] (1) --> localOut (2)
```
