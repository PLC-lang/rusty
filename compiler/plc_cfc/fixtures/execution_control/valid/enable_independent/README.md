What: a data chain where each block carries its own EN gate. The guards stay
independent (`t1` and `t2`), and `b`'s input read of `a.out` sits inside `b`'s
guard but outside `a`'s, so a disabled `a` feeds its buffered output to a
running `b`.

Illustrated:
```
              a : counter (0)            b : counter (1)
            +-----------------+        +-----------------+
     t1 --> | EN              | t2 --> | EN              |
   seed --> | in          out | --in-> |             out | --> result (2)
            +-----------------+        +-----------------+
```
