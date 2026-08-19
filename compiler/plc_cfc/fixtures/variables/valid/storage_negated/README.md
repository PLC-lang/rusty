What: negation bubbles composing with storage modes, one row per flavor —
plain Set, sink bubble, source bubble (with Reset), and both bubbles. Each
bubble wraps the guarding condition in one `NOT`; the stored constant is
untouched: `IF NOT NOT a THEN b := TRUE`.

Illustrated:
```
a --> [b |S] (0)
a --o [b |S] (1)
a o-> [b |R] (2)
a o-o [b |S] (3)
```
