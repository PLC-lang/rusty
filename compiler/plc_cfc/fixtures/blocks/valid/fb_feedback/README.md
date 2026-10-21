What: A function-block instance whose `out` pin feeds back into its own `in`
pin (1) — `inst(in := inst.out)`. Same shape as `program_feedback`, but read
through the instance rather than the type; the read resolves to the instance's
persistent member, so no ordering fix-up or temporary is needed.

Illustrated:
```
            inst : counter (1)
          +-------------------+
   .----> | in            out | --.
   |      +-------------------+   |
   '------------------------------'   [out feeds back into in]
```
