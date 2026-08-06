What: a builtin `ADD` whose output feeds back into its own input; the
accumulator temporary must resolve to DINT through the feedback binding.
`main` seeds `2^24 + 1` (not representable in REAL) and runs three cycles —
the exact sum (`acc = 50331651`) proves no floating-point promotion happened.

Illustrated:
```
     +--<-- ADD <--+
     |             |
     +--> [ADD]----+--> acc (0, 1)
seed ---->
```
