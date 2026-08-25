What: two blocks feed each other's EN from the other's ENO, so neither EN wire
ever reaches a real source. The ENO see-through trace detects the revisit and
rejects both blocks, mirroring the connector cycle guard.

Illustrated:
```
       +----------------------------------------------------+
       |     a : counter (0)            b : counter (1)     |
       |   +-----------------+        +-----------------+   |
       +-> | EN          ENO | -----> | EN          ENO | --+
seed --+-> | in          out |   +--> | in          out |
       |   +-----------------+   |    +-----------------+
       +-------------------------+
       (both out pins unread; no EN wire reaches a source)
```
