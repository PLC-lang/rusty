What: A function-block instance call. The block carries `instanceName="inst"`
(the caller's `inst : counter;`), so the call and the output read run on the
instance — `inst(in := localIn); localOut := inst.out;` — not on the type. This
is the sole difference from a program call: the reference is the instance, not
the singleton type name.

Illustrated:
```
localIn --> in [inst : counter] out (0) --> localOut (1)
```
