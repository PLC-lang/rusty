What: two instances of the same function block called in one network; each
keeps its own state. `main` runs two cycles and checks the accumulators stayed
separate (`a = 2, b = 20` from `sa = 1`, `sb = 10`).

Illustrated:
```
         a : counter (1)
       +--------------+
sa --> | step   total | --> outA (2)
       +--------------+

         b : counter (3)
       +--------------+
sb --> | step   total | --> outB (4)
       +--------------+
```
