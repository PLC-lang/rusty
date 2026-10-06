What: a sink with execution order 0. The IDE numbers execution orders from 1,
and order 0 would map to debug line 0 ("no source line"), so this is rejected
with an "invalid execution order" error (E158). The sink with order 1 is valid.

Illustrated:
```
foo --+--> bar (0)    (order 0, rejected)
      +--> baz (1)
```
