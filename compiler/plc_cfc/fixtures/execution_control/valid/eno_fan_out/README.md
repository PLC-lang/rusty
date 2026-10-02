What: the ENO pin fans out to two sinks. Every consumer resolves transparently
to the EN source; re-evaluating `trigger` twice is safe because sources are
vetted to literals and references.

Illustrated:
```
              inst : counter (1)
            +-------------------+
trigger --> | EN            ENO | --+--> d1 (2)
localIn --> | in            out |   '--> d2 (3)
            +-------------------+
            (out unread)
```
