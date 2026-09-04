What: negation bubbles on `Reference` sinks, one row per side. An address has
no negation, so each row is rejected with E154 instead of transpiling to a
`REF=` with a synthesized `NOT`.

Illustrated:
```
a o-> [b |REF] (0)
a --o [b |REF] (1)
```
