# generic_unbound_feedback

What: A generic block whose generic-typed input is fed only by its own
feedback loop, while a *non-generic* input (`a : DINT`) is wired to a
concrete source. The concrete input makes the call look decidable, but it
does not bind `T` — the only candidate for `T` is the still-open temporary
itself, so nothing concrete ever decides the binding and the block reports
E149. (Without the emptiness check the smallest possible type of the nature,
`USINT` for `ANY_NUM`, would win by default and the accumulator would
silently wrap at 255.)

Illustrated:
```
        myGenScale<T: ANY_NUM> : T     (a : DINT decides nothing for T)
       +---------------------+
seed-->| a        myGenScale | --+--> acc
  .--> | b                   |   |
  |    +---------------------+   |
  '------------------------------'
```
