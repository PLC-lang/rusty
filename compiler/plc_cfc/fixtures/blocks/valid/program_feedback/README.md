What: A single program block whose `out` pin feeds straight back into its own
`in` pin (0). The read resolves to the program's persistent member, so the call
lowers to `counter(in := counter.out)` with no ordering fix-up or temporary.

Illustrated:
```
   +--> in [counter] out (0) --+
   |                           |
   +---------------------------+
```
