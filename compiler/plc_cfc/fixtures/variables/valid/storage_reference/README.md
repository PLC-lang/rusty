What: a sink with storage mode `Reference`. The sink re-points instead of
assigning: it stores the source's address, `b REF= a`, so later reads of `b`
see whatever value `a` holds at that time.

Illustrated:
```
a --> [b |REF] (0)
```
