What: `a`'s ENO chains into `b`'s EN, so both calls share the same enable. The
trace sees through the ENO pin to `a`'s EN source (like a connector hop), so
both guards and the final `done` sink all read `trigger` directly.

Illustrated:
```
              a : counter (1)            b : counter (2)
            +-----------------+        +-----------------+
trigger --> | EN          ENO | -----> | EN          ENO | --> done (4)
   seed --> | in          out | -----> | in          out | --> result (3)
            +-----------------+        +-----------------+
```
