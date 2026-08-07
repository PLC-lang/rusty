What: a sink with storage mode `Reset`, the counterpart of `storage_set`. The
incoming value guards a constant `FALSE` store: `IF a THEN b := FALSE; END_IF`.
Nothing is written while `a` is `FALSE`, so `b` keeps its value until reset.

Illustrated:
```
a --> [b |R] (0)
```
