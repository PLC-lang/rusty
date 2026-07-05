# unconnected_arguments_function_block

The same partly-wired block as `unconnected_arguments_function`, but the block targets
a **function block instance**. An unconnected input or in-out pin is still emitted as a
named argument with an **empty right-hand side** (`b := `, `io := `) so the call keeps
its full argument list.

```text
                  +------- myFb -------+ (1)
   localA ------->| a                  |
                  | b  (unconnected)   |
                  | io (unconnected)   |
                  +--------------------+

   myFb           called on instance myInstance (the block's instanceName)
   (unconnected)  a pin with no incoming wire
   (1)            evaluation-priority badge shown by the IDE
```

- `mainProgram.cfc` — the program under test; it owns the instance `myInstance : myFb`.
- `myFb.st` — the function block it calls; `a`, `b` (default `99`) are inputs, `io` is an in-out.

Only `a` is wired, and the block is called on `myInstance`, so the network means:

```text
myInstance(a := localA, b := , io := );
```

The empty `b` falls back to its declared default (`99`); the empty in-out `io` is left
for downstream validation (`E031`). The transpiler lowers both faithfully.
