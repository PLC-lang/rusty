What: A block call interleaved with a plain variable wire, ordered purely by
priority: `q := p` (0), `r := counter.out` (1), then the `counter` call (2). Both
reads precede the call, so `r` observes last cycle's output. Confirms `Call` and
`Assignment` statements sort into one priority-ordered list.

Illustrated:
```
seed --> in [counter] out (2) --> r (1)
p --> q (0)
```
