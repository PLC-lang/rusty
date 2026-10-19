What: a VOID function assigns to itself, i.e. the sink is named after the POU.
The function has no return value, so the assignment is rejected (E037). The
message names the left side with its text from the declaration, not with text
from the XML document at the same offsets.

Illustrated:
```
x --> void_self_assignment (1)    (rejected)
```
