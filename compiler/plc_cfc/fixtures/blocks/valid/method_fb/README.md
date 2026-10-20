What: A call to a function-block **method**. The block's `typeName` is qualified
(`counter.add`) and `instanceName="inst"`, so the call dispatches to the method,
`inst.add(in := localIn, ...)`. Unlike an action's, a method's return value and
outputs do not survive the call, so both are captured into temporaries during
the call, like a function's: `__out_counter_add_3` and `__out_doubled_3`.

Illustrated:
```
localIn --> in [inst : counter.add] (1) --> localOut (2)
                                doubled --> localDoubled (3)
```
