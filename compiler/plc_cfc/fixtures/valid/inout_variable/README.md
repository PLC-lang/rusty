# inout_variable

A function call with a `VAR_IN_OUT` parameter. The in-out pin (`sum`) binds a
variable by reference — it is both read and written in place by the call.

```text
                      +---- accumulate ----+ (1)
   localValue  ------>| value              |
                      |          accumulate|--->  localResult  (2)
   localSum  <------->| sum                |
                      +--------------------+

   <-->     an in-out pin (passed by reference)
   (1),(2)  evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test (the FBD network above).
- `accumulate.st` — the function it calls (input `value`, in-out `sum`).

`localValue` feeds the `value` input and `localSum` is bound to the in-out `sum`,
so the network means:

```text
__accumulate_res_0 := accumulate(value := localValue, sum := localSum);
localResult := __accumulate_res_0;
```
