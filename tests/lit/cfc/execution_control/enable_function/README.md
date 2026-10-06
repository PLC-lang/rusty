What: a function call guarded by EN. The whole captured call sits inside the
guard, so while EN is FALSE neither the function nor the `__out_...` capture
runs: `sum` yields the last recorded return value (default 0 if EN was never
TRUE), even after the inputs change.

Illustrated:
```
                  myAdd (1)
            +--------------------+
trigger --> | EN             ENO | --> done (3)
      a --> | in1          myAdd | --> sum (2)
      b --> | in2   myAddDoubled |
            +--------------------+
            (myAddDoubled unread)
```
