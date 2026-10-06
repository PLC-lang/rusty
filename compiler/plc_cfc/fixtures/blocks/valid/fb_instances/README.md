What: Two distinct instances of the same function block, `a` (1) then `b` (2),
chained `seed --> a --> b --> result` (3). Unlike a program (a shared singleton),
each instance holds its own state, so `a.out` and `b.out` are separate — the
degenerate case for programs becomes meaningful here.

Illustrated:
```
seed --> in [a : counter] out (1) --> in [b : counter] out (2) --> result (3)
```
