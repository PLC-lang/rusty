# function_block_call

A block that targets a **function block instance**. Unlike a function call, an FB
block carries an `instanceName` and is called on that instance; it has no return
value, so its result leaves through a named `VAR_OUTPUT` pin.

```text
                  +------ Counter ------+ (1)
   localStep ---->| step          count |---->  localCount  (2)
                  +---------------------+

   Counter   called on instance myInstance (the block's instanceName)
   (1),(2)   evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test; it owns the instance `myInstance : Counter`.
- `Counter.st` — the function block it calls (input `step`, output `count`).

`localStep` feeds the instance's `step` input. The block has no return pin (its
output is the named `count` pin, not a pin named after the type), so it is called
on `myInstance` and its `count` output is routed through a temporary that the
`localCount` sink then reads:

```text
myInstance(step := localStep, count => temp_0);
localCount := temp_0;
```
