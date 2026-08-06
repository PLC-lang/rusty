What: a user-defined generic function whose return value feeds back into its
own input; the temporary's type must resolve through the feedback loop. `main`
runs three cycles with `seed = 5` and checks the accumulation (`acc = 15`).

Illustrated:
```
     +--<-- myGenAdd <--+
     |                  |
     +--> a [myGenAdd]--+--> acc (0, 1)
seed ---> b
```
