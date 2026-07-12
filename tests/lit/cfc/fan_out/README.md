# fan_out

Integration test: a **block output that fans out to several consumers is evaluated once**.
`Tick`'s single output feeds two sinks (`outA`, `outB`). A CFC block runs exactly once, so
the producer must be evaluated into one temporary that both sinks then read — not inlined
(and re-run) per consumer.

```text
                    +------ Tick ------+  (0)
   seed ----------->| seed        Tick |---+------------>  outA  (1)
                    +------------------+   |
                                           +------------>  outB  (2)

   Tick      a side-effecting ST function (tick.st): bumps the global `calls`, returns seed
   (0),(1,2) evaluation-priority badges shown by the IDE
```

- `tick.st` — `VAR_GLOBAL calls` and `FUNCTION Tick` (`calls := calls + 1; Tick := seed;`).
- `fanout.cfc` — `PROGRAM FanOut` (`seed`, `outA`, `outB`). `Tick`'s output wires to both sinks.
- `main.st` — entry point: runs `FanOut` with `seed := 7`, prints `outA`, `outB`, and `calls`.

Lowers to a single evaluation reused by both sinks:

```text
__Tick_res_0 := Tick(seed := seed);
outA := __Tick_res_0;
outB := __Tick_res_0;
```

Both sinks read `7`, and `calls = 1` (not `2`) proves `Tick` ran exactly once.
