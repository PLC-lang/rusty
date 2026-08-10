What: the three negation-bubble flavors on plain assignments — source bubble
(`b1 := NOT a1`), sink bubble (`b2 := NOT a2`), and both (`b3 := NOT NOT a3`,
the NOTs cancel). `main` runs an all-false and an all-true round and checks
each sink.

Illustrated:
```
a1 o--> b1 (0)
a2 --o> b2 (1)
a3 o-o> b3 (2)
```
