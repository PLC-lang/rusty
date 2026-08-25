What: a block declares execution control but its EN pin carries no connection.
A call whose enable can never be decided is rejected; unlike an unwired data
input there is no sensible default.

Illustrated:
```
              inst : counter (0)
            +-------------------+
        ?-- | EN                |
localIn --> | in            out | --> localOut (1)
            +-------------------+
            (EN has no incoming connection)
```
