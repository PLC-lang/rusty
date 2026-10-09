What: A block call interleaved with a plain variable wire, ordered purely by
priority: `q := p` (1), `r := counter.out` (2), then the `counter` call (3). Both
reads precede the call, so `r` observes last cycle's output. Confirms `Call` and
`Assignment` statements sort into one priority-ordered list.

Illustrated:
```
seed --> in [counter] out (3) --> r (2)
p --> q (1)
```
