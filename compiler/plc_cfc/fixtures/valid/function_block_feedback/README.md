# function_block_feedback

A **direct self-feedback wire** (design doc FB3, the integrator pattern): the
block's output pin is wired straight back to its own input. The consumed output
gets a persistent result variable (plain `VAR`, not `VAR_TEMP`), so the input
reads the *previous cycle's* result — the call's `in1 :=` argument binds before
the body runs.

```text
           +------ Counter ------+ (0)
   +------>| in1            out1 |-------+-->  y  (1)
   |       +---------------------+       |
   +-------------------------------------+

   Counter  called on instance myInstance
   (0),(1)  evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test (the FBD network above).
- `Counter.st` — the function block it calls (`out1 := in1 + 1`).

The network means:

```text
myInstance(in1 := __Counter_res_0, out1 => __Counter_res_0);
y := __Counter_res_0;
```

Because `__Counter_res_0` lives in `VAR` (static in a program body), each cycle
increments: y = 1, 2, 3, ... — the runtime behavior is pinned by the
`function_block_feedback` lit test.
