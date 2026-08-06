What: a call to a function-block action (`typeName="counter.bump"` on the
instance `inst`); the action mutates the instance's state. `main` runs two
cycles with `amount = 3` and checks the accumulated count (`count = 6`).

Illustrated:
```
amount --> step [inst.bump] count --> result (0, 1)
```
