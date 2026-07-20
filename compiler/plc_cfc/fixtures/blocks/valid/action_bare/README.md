What: A parameterless action call — `inst.reset()`. The block has no input,
in_out, or output pins, so the call carries no arguments; it still emits (the
action runs for its effect on the instance's state). Confirms a bare qualified
call lowers cleanly.

Illustrated:
```
[inst : counter.reset] (0)
```
