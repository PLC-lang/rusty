What: a function whose return value feeds back into its own input pin; the
captured temporary carries last cycle's value. `main` runs two cycles with
`seed = 5` and checks the accumulation (`acc = 10`).

Illustrated:
```
              myAdd (1)
            +--------------------+
     .----> | in1          myAdd | --+--> acc (2)
seed -----> | in2   myAddDoubled |   |
            +--------------------+   |
     '-------------------------------'   [return feeds back into in1]
```
