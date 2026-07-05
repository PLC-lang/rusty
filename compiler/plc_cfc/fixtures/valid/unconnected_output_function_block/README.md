# unconnected_output_function_block

The same unconnected-output case as `unconnected_output_function`, but the block targets a
**function block instance**. The output `result` feeds nothing, so it is emitted as a named
argument with an empty right-hand side (`result => `).

```text
                  +------- myFb -------+ (1)
   localA ------->| a           result |   (result unconnected)
                  +--------------------+

   myFb     called on instance myInstance (the block's instanceName)
   result   an output pin with no outgoing wire
   (1)      evaluation-priority badge shown by the IDE
```

- `mainProgram.cfc` — the program under test; it owns the instance `myInstance : myFb`.
- `myFb.st` — the function block it calls; `a` is an input, `result` is an output.

The block is called on `myInstance` and its unconnected output is emitted empty, so the
network means:

```text
myInstance(a := localA, result => );
```
