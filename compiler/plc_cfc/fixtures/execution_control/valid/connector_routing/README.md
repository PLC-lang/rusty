What: EN and ENO wired through connector pairs. The trace must compose its two
hop kinds: the EN wire hops continuation-to-connector before it reaches
`trigger`, and the `done` sink hops continuation-to-connector before it reaches
the ENO pin, then through the block onto the same EN wire.

Illustrated:
```
trigger --> x>

              inst : counter (1)
            +-------------------+
     >x --> | EN            ENO | --> y>
localIn --> | in            out | --> localOut (2)
            +-------------------+

>y --> done (3)
```
