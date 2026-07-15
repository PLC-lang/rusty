What: Two distinct program blocks in series — `counter` (0) then `doubler` (1) —
with `counter.out` wired into `doubler.in` and `doubler.out` read into `result`
(2). A block input fed by another block's output lowers to a member access inside
the call arguments.

Illustrated:
```
seed --> in [counter] out (0) --> in [doubler] out (1) --> result (2)
```
