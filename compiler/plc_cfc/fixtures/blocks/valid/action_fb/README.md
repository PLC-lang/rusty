What: A call to a function-block **action**. The block's `typeName` is qualified
(`counter.increment`) and `instanceName="inst"`, so the call dispatches to the
action — `inst.increment(in := localIn)` — while its output is read off the
instance itself, `localOut := inst.out` (outputs are the parent's members, not
the action's). Owner/action come from `rsplit_once('.')` on `typeName`.

Illustrated:
```
localIn --> in [inst : counter.increment] out (0) --> localOut (1)
```
