What: The return pin of `myAdd` feeds back into its own `in1` (an accumulator);
`in2` takes `seed`, and `acc` reads the return. The input reads the persisted
temporary (last cycle's return) before the call overwrites it, so a stateless
self-cycle resolves cleanly with no reordering and no cycle error.

Illustrated:
```
              myAdd (0)
            +--------------------+
     .----> | in1          myAdd | --+--> acc (1)
seed -----> | in2   myAddDoubled |   |
            +--------------------+   |
     '-------------------------------'   [return feeds back into in1]
```
