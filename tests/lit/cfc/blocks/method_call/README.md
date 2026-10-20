What: a call to a function-block method (`typeName="counter.bump"` on the
instance `inst`); the method adds to the instance's state and returns the new
total. Its return value and its output `doubled` do not survive the call, so
the call captures both into temporaries. `main` runs two cycles with
`amount = 3` and checks both values (`result = 6, doubled = 12`).

Illustrated:
```
amount --> step [inst.bump] (1) --> result (2)
                        doubled --> dbl (3)
```
