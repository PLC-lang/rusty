What: a builtin `ADD` whose output feeds back into its own input; the
accumulator temporary must resolve to DINT through the feedback binding.
`main` seeds `2^24 + 1` (not representable in REAL) and runs three cycles —
the exact sum (`acc = 50331651`) proves no floating-point promotion happened.

Illustrated:
```
              ADD (1)
            +-----------+
     .----> | IN1   ADD | --+--> acc (2)
seed -----> |           |   |
            +-----------+   |
     '----------------------'   [return feeds back into IN1]
```
