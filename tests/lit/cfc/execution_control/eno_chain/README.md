What: the ENO cascade at runtime. `a`'s ENO gates `b`, so one signal switches
the whole chain: both run while `trigger` is TRUE, both hold while it is FALSE,
and `done` mirrors the shared enable.

Illustrated:
```
              a : counter (1)            b : counter (2)
            +-----------------+        +-----------------+
trigger --> | EN          ENO | -----> | EN          ENO | --> done (4)
   seed --> | in          out | -----> | in          out | --> result (3)
            +-----------------+        +-----------------+
```
