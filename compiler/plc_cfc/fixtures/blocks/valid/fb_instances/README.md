What: Two distinct instances of the same function block, `a` (0) then `b` (1),
chained `seed --> a --> b --> result` (2). Unlike a program (a shared singleton),
each instance holds its own state, so `a.out` and `b.out` are separate — the
degenerate case for programs becomes meaningful here.

Illustrated:
```
seed --> in [a : counter] out (0) --> in [b : counter] out (1) --> result (2)
```
