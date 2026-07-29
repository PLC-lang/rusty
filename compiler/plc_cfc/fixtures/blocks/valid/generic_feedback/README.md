# generic_feedback

What: The generic `myGenAdd<T: ANY_NUM>` as an accumulator — its return pin
feeds back into its own `b` while `a` takes `seed`. The self-candidate binds
nothing (its type is the binding under inference); `seed` alone decides
`T = DINT`. The self input is wired second on purpose: a generic candidate
after the concrete one must not override it.

Illustrated:
```
            myGenAdd (0)
          +-----------------+
seed ---> | a      myGenAdd | --+--> acc (1)
   .----> | b               |   |
   |      +-----------------+   |
   '----------------------------'   [return feeds back into b]
```
