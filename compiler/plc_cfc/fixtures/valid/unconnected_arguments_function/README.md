# unconnected_arguments_function

A block (here a **function**) whose pins are only partly wired. An unconnected input
or in-out pin is still emitted as a named argument with an **empty right-hand side**
(`b := `, `io := `) rather than being dropped, so the call keeps its full argument
list. This is exactly the AST the ST parser builds for `myFunc(a := localA, b := , io := )`.

```text
                  +------ myFunc ------+ (1)
   localA ------->| a           myFunc |------->  localResult  (2)
                  | b  (unconnected)   |
                  | io (unconnected)   |
                  +--------------------+

   (unconnected)  a pin with no incoming wire
   (1),(2)        evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test (the FBD network above).
- `myFunc.st` — the function it calls; `a`, `b` (default `99`) are inputs, `io` is an in-out.

Only `a` is wired. The transpiler still emits `b` and `io`, so the network means:

```text
__myFunc_res_0 := myFunc(a := localA, b := , io := );
localResult := __myFunc_res_0;
```

Downstream the empty `b` falls back to its declared default (`99`), while the empty
in-out `io` is rejected by validation (`E031` — an in-out needs a reference). The
transpiler lowers both faithfully and leaves that judgement to the validator.
