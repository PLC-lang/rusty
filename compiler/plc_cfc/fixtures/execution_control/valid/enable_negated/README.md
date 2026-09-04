What: an inversion bubble on the EN pin. The guard becomes `NOT trigger`, and
because ENO mirrors the post-negation EN value, the ENO consumer also reads
`NOT trigger`.

Illustrated:
```
              inst : counter (0)
            +-------------------+
trigger --o | EN            ENO | --> done (2)
localIn --> | in            out | --> localOut (1)
            +-------------------+
            (o marks the inversion bubble on EN)
```
