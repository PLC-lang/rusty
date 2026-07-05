# unconnected_output_program

The same unconnected-output case as `unconnected_output_function`, but the block targets a
**program**. The output `result` feeds nothing, so it is emitted as a named argument with an
empty right-hand side (`result => `).

```text
                      +---- auxProgram ----+ (1)
   localA ----------->| a           result |   (result unconnected)
                      +--------------------+

   result  an output pin with no outgoing wire
   (1)     evaluation-priority badge shown by the IDE
```

- `mainProgram.cfc` — the program under test (the FBD network above).
- `auxProgram.st` — the program it calls; `a` is an input, `result` is an output.

The program is called by name (no instance) and its unconnected output is emitted empty, so
the network means:

```text
auxProgram(a := localA, result => );
```
