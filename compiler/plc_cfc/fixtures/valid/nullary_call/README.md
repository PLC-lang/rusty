# nullary_call

A function with no inputs. Its result is still evaluated once into a temporary
that the sink reads — exercising the no-argument call form `getOffset()`.

```text
                    +--- getOffset ---+ (1)
                    |       getOffset |--->  localResult  (2)
                    +-----------------+

   (1),(2)  evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test (the FBD network above).
- `getOffset.st` — the function it calls (`FUNCTION getOffset : DINT`, no inputs).

`getOffset` takes no arguments; its result flows into `localResult`, so the
network means:

```text
temp_0 := getOffset();
localResult := temp_0;
```
