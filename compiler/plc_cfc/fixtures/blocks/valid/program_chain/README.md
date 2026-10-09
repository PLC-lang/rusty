What: Two distinct program blocks in series — `counter` (1) then `doubler` (2) —
with `counter.out` wired into `doubler.in` and `doubler.out` read into `result`
(3). A block input fed by another block's output lowers to a member access inside
the call arguments.

Illustrated:
```
seed --> in [counter] out (1) --> in [doubler] out (2) --> result (3)
```
