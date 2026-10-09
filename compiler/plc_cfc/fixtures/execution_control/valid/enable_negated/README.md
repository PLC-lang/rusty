What: an inversion bubble on the EN pin. The guard becomes `NOT trigger`, and
because ENO mirrors the post-negation EN value, the ENO consumer also reads
`NOT trigger`.

Illustrated:
```
              inst : counter (1)
            +-------------------+
trigger --o | EN            ENO | --> done (3)
localIn --> | in            out | --> localOut (2)
            +-------------------+
            (o marks the inversion bubble on EN)
```
