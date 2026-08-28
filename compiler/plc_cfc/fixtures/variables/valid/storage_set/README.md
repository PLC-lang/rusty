What: a sink with storage mode `Set`. The incoming value no longer assigns
but guards a constant store: `IF a THEN b := TRUE; END_IF`. Nothing is
written while `a` is `FALSE`, so a once-set `b` stays latched.

Illustrated:
```
a --> [b |S] (0)
```
