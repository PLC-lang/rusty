# action

Integration test: a **CFC block targeting an action**. `Runner` (a CFC program) owns an
instance of the `Latch` function block and calls its `fire` action — a restricted method
with no inputs, outputs, or return value that only mutates the FB's internal `state`.
The program then routes that internal state out to its `out` variable so it can be observed.

```text
   +------ inst.fire ------+   (0)        the action call (no pins): mutates inst.state
   |   (action, no I/O)    |
   +----------------------+

   inst.state  ------------------>  out   (1)   copy the mutated state out

   inst.fire   the action `fire` of the `Latch` instance `inst` (qualified type name)
   (0),(1)     evaluation-priority badges shown by the IDE
```

- `latch.st` — `FUNCTION_BLOCK Latch` with internal `state : DINT` and `ACTION fire` (`state := 5`).
- `runner.cfc` — the CFC program: fires `inst.fire()` (priority 0), then `out := inst.state` (priority 1).
- `main.st` — entry point: runs `Runner`, prints `Runner.out`.

The action block lowers to a qualified member call `inst.fire();`. Observing `5` (not the
default `0`) proves the action resolved and ran.
