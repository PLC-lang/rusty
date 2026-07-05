# program_call

A block that targets a **program**. A program is a global singleton, so the block
carries no `instanceName` and is called by its name, like a function call but with
no return value; its result leaves through a named `VAR_OUTPUT` pin.

```text
                       +----- auxProgram -----+ (1)
   localIncrement ---->| increment      total |---->  localTotal  (2)
                       +----------------------+

   (1),(2)  evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test (the FBD network above).
- `auxProgram.st` — the program it calls (input `increment`, output `total`).

`localIncrement` feeds `auxProgram`'s `increment` input. A program has no return
pin, so it is called by name and its `total` output is routed through a temporary
that the `localTotal` sink then reads:

```text
auxProgram(increment := localIncrement, total => temp_0);
localTotal := temp_0;
```
