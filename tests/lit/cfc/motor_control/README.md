# motor_control

The **real-world capstone**: a motor start/stop controller with a seal-in latch and an
emergency-stop interlock, built from multiple **mixed CFC + ST files** and driven by a
single ST `main`. It exercises, in one project, most of the supported CFC feature set:

- a CFC **FUNCTION** (`StartCmd`) and a CFC **FUNCTION_BLOCK** (`Motor`);
- **ST functions called from CFC** (`MyAnd`, `MyOr` — operator-block stand-ins, see `../OPEN.md`);
- an **ST function block called from CFC** (`SrLatch`, stateful);
- a **CFC function called from CFC** (`Motor` → `StartCmd`);
- a **negated input** (`NOT estop` inside `StartCmd`);
- a **connector/continuation** routing `estop` to two consumers (a named virtual wire);
- **temp-type inference** for every block output (`__return@StartCmd`, `__return@MyOr`, `__output@SrLatch@q`);
- **ST calling CFC** (`main` drives the `Motor` instance).

## Files

- `logic.st` — `MyAnd`, `MyOr`, and the reset-dominant `SrLatch` function block.
- `start_cmd.cfc` — CFC `FUNCTION StartCmd : BOOL`, computes `start AND NOT estop`.
- `motor.cfc` — CFC `FUNCTION_BLOCK Motor`, orchestrates the controller.
- `main.st` — entry point: drives the `Motor` through a scan sequence and prints `running`.

## `start_cmd.cfc` — the start interlock

```text
   start ─────────►┌──── MyAnd ────┐
                   │ a        MyAnd │──►  StartCmd      StartCmd := start AND NOT estop
   estop ─(NOT)───►│ b             │
                   └───────────────┘
```

## `motor.cfc` — the controller

`estop` enters a `Connector` labelled `estop_wire`; the matching `Continuation` re-emits
that wire to **both** `StartCmd.estop` and `MyOr.b`. `StartCmd` produces the latch `set`,
`MyOr` (`stop OR estop`) produces the reset, and the `SrLatch` instance holds `running`.

```text
   start ──────────────────────────►┌──── StartCmd ────┐
                                     │ start            │
              ┌── estop_wire ┄┄┄┄┄┄►│ estop (NOT)  out │──┐ setCmd
              ╎   (continuation)     └──────────────────┘  │
   estop ────►┴ estop_wire                                 ▼
              ╎   (connector)       ┌──── MyOr ────┐   ┌── latch : SrLatch ──┐
   stop ──────╎────────────────────►│ a        MyOr│──►│ set              q  │──► running
              └┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄►│ b            │   │ reset               │
                                    └──────────────┘   └─────────────────────┘
                                            resetCmd
```

Lowers (in evaluation-priority order) to:

```text
__temp_0 := StartCmd(start := start, estop := estop);   (* set  = start AND NOT estop *)
__temp_1 := MyOr(a := stop, b := estop);                (* reset = stop OR estop      *)
latch(set := __temp_0, reset := __temp_1, q => __temp_2);
running := __temp_2;
```

## The scan sequence (`main`)

The `Motor` instance retains its latch between calls, so successive scans behave like a PLC:

1. `start` pressed → seal-in latches the motor **on** (`after_start = 1`).
2. `start` released → still running, the latch holds (`still_running = 1`).
3. `stop` pressed → motor **off** (`after_stop = 0`).
4. restart, then `estop` asserted → interlock keeps it **off** (`after_estop = 0`).
