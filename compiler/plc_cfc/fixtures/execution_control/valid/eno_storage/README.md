What: two AddData extensions composed on one wire: the ENO pin feeds a sink
with Set storage mode. The transparent ENO source becomes the latch guard, so
`latch` engraves `TRUE` on the first enabled cycle and never resets.

Illustrated:
```
              inst : counter (0)
            +-------------------+
trigger --> | EN            ENO | --S--> latch (1)
localIn --> | in            out |
            +-------------------+
            (out unread; S marks the Set storage mode)
```
