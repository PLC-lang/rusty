What: a function with an inout parameter wired to a program variable; the
callee reads and writes `total` through the pin, and the return value lands in
`result`. `main` runs two cycles and checks the write-back stuck
(`total = 10, result = 10` for `delta = 5`).

Illustrated:
```
             addInto (1)
           +--------------------+
     5 --> | delta      addInto | --> result (2)
total <--> | acc                |
           +--------------------+
```
