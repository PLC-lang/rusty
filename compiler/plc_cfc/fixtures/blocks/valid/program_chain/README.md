What: Two distinct program blocks in series — `counter` (1) then `doubler` (2) —
with `counter.out` wired into `doubler.in` and `doubler.out` read into `result`
(3). A block input fed by another block's output lowers to a member access inside
the call arguments.

Illustrated:
```
           counter (1)        doubler (2)
         +------------+     +------------+
seed --> | in     out | --> | in     out | --> result (3)
         +------------+     +------------+
```
