What: inversion bubbles on both the EN and the ENO pin. The guard reads
`NOT trigger`; the ENO consumer sees the post-negation EN value negated once
more by the ENO bubble, so the two bubbles cancel and `done` reads `trigger`.

Illustrated:
```
              inst : counter (1)
            +-------------------+
trigger --o | EN            ENO | o--> done (3)
localIn --> | in            out | ---> localOut (2)
            +-------------------+
            (o marks the inversion bubbles)
```
