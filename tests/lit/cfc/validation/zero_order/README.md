What: the sink `bar` has execution order 0. The IDE numbers execution orders
from 1, and a debugger reads line N of the diagram's debug file as order N, so
order 0 would be line 0 ("no source line"). The element is rejected (E158).

Illustrated:
```
foo --+--> bar (0)    (rejected)
      +--> baz (1)
```
