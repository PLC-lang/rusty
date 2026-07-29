# generic_call

What: The generic function `myGenAdd<T: ANY_NUM>` (an `ADD`-shaped callee)
called with two DINT inputs; its return pin feeds the enclosing function's
result. The captured temporary must take the call's concrete type (DINT), not
the callee's unresolved generic return type.

Illustrated:
```
          myGenAdd (0)
        +-----------------+
in1 --> | a      myGenAdd | --> generic_call (1)
in2 --> | b               |
        +-----------------+
```
