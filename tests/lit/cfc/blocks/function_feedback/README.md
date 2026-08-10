What: a function whose return value feeds back into its own input pin; the
captured temporary carries last cycle's value. `main` runs two cycles with
`seed = 5` and checks the accumulation (`acc = 10`).

Illustrated:
```
     +--<-- myAdd <--+
     |               |
     +--> in1 [myAdd]+--> acc (0, 1)
seed ---> in2
```
