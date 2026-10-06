What: a function-block call guarded by EN, with ENO wired to `done`. Three
cycles verify the runtime contract: while EN is FALSE the block never runs and
`out` keeps its default; a TRUE cycle runs the block and sets `done`; dropping
EN again holds the last recorded `out` and clears `done`.

Illustrated:
```
              inst : counter (1)
            +-------------------+
trigger --> | EN            ENO | --> done (3)
localIn --> | in            out | --> localOut (2)
            +-------------------+
```
