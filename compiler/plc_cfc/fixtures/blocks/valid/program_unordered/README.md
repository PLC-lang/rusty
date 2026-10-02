What: The same wiring as `program_call`, but the sink reading the output has a
lower priority number than the block that produces it — sink `countOut` (1),
block `counter` (2). Statements stay in raw priority order, so the read is
emitted *before* the call and simply observes the member's previous-cycle value.
This is exactly why stateful outputs need no reordering and no temporary.

Illustrated:
```
countIn --> in [counter] out (2) --> countOut (1)
```
