What: a call to a function without a return value; only the declared output is
captured. `main` checks the output arrived (`result = 6` for `localIn = 3`).

Illustrated:
```
              myVoid (1)
            +--------------+
localIn --> | in       out | --> localOut (2)
            +--------------+
```
