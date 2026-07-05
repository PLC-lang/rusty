# unconnected_arguments_program

The same partly-wired block as `unconnected_arguments_function`, but the block targets
a **program**. An unconnected input or in-out pin is still emitted as a named argument
with an **empty right-hand side** (`b := `, `io := `) so the call keeps its full
argument list.

```text
                      +---- auxProgram ----+ (1)
   localA ----------->| a                  |
                      | b  (unconnected)   |
                      | io (unconnected)   |
                      +--------------------+

   (unconnected)  a pin with no incoming wire
   (1)            evaluation-priority badge shown by the IDE
```

- `mainProgram.cfc` — the program under test (the FBD network above).
- `auxProgram.st` — the program it calls; `a`, `b` (default `99`) are inputs, `io` is an in-out.

Only `a` is wired, and the program is called by name (no instance), so the network means:

```text
auxProgram(a := localA, b := , io := );
```

The empty `b` falls back to its declared default (`99`); the empty in-out `io` is left
for downstream validation (`E031`). The transpiler lowers both faithfully.
