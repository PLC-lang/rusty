What: A block calls the function block action `counter.increment` but names no
`instanceName`, so the action has no instance to run on and the block is
rejected (E159). A program action (`blocks/valid/action_program`) needs no
instance, because the program is its own.

Illustrated:
```
              counter.increment (1)
            +-------------------+
localIn --> | in            out | --> localOut (2)
            +-------------------+
            (no instance: rejected)
```
