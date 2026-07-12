# feedback_in_function

The same direct feedback wire as `function_block_feedback`, but inside a
**FUNCTION**-bodied POU. The emission is identical — the result variable lands
in a plain `VAR` block — but a function's `VAR` is per-invocation storage, so
the feedback wire deterministically reads the type default (`0`) on every call
instead of the previous cycle's value: semantically questionable to draw, but
well-defined, and the network transpiles and compiles without diagnostics
(see the design doc, `docs/evaluation_priorities.md`).

```text
           +-------- inc --------+ (0)
   +------>| x               inc |-------+-->  myFunc  (1)
   |       +---------------------+       |
   +-------------------------------------+

   myFunc   the FUNCTION's return value (a sink named after the function)
   (0),(1)  evaluation-priority badges shown by the IDE
```

- `myFunc.cfc` — the function under test (the FBD network above).
- `inc.st` — the function it calls (`inc := x + 1`).

The network means:

```text
__inc_res_0 := inc(x := __inc_res_0);
myFunc := __inc_res_0;
```

`__inc_res_0` is re-initialized per call, so `myFunc()` returns 1 on every
invocation — pinned by the `feedback_in_function` lit test.
