What: two `myAdd` calls in series; the first return value feeds the second
call's input, captured through a temporary. `main` checks the chained sum
(`(seed + k) + k = 11` for `seed = 3, k = 4`).

Illustrated:
```
           myAdd (1)                      myAdd (2)
         +--------------------+         +--------------------+
seed --> | in1          myAdd | ------> | in1          myAdd | --> result (3)
   k --> | in2   myAddDoubled |   k --> | in2   myAddDoubled |
         +--------------------+         +--------------------+
         (myAddDoubled unread on both blocks)
```
