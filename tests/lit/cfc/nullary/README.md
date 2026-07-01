# nullary

Integration test: a **no-input function call** whose result is captured. The CFC program
`Offset` calls the ST function `getOffset` (no inputs) and wires its return to a sink.
Exercises the nullary call form `getOffset()` and the result-pin temp at runtime — the
`action` test also has an empty-input block, but an action has no return/temp.

```text
                    +--- getOffset ---+ (0)
                    |       getOffset |--->  result  (1)
                    +-----------------+

   (0),(1)  evaluation-priority badges shown by the IDE
```

- `get_offset.st` — `FUNCTION getOffset : DINT` (no inputs): `getOffset := 100;`.
- `offset.cfc` — `PROGRAM Offset` calling `getOffset` into `result`.
- `main.st` — entry point: runs `Offset`, prints `result`.

Lowers to:

```text
__temp_0 := getOffset();
result := __temp_0;
```

Observing `100` proves the no-argument call runs and its result flows through the temp.
