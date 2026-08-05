# variadic_unwired

What: The builtin variadic `ADD` with three pins whose *middle* pin is left
unwired (a `ConnectionPointIn` without a `Connection`, the shape the IDE
exports — see `reference/and_unwired.cfc`, from which this fixture is derived
by rename). A positional list has no empty slot, so the dangling pin is
dropped and the call passes the two wired inputs: `ADD(x, z)`.

Illustrated:
```
        ADD (0)
      +---------------+
x --> | IN1       ADD | --> result (1)
      |     (unwired) |
z --> |               |
      +---------------+
```
