# mux_call

What: The builtin variadic `MUX` whose declaration mixes a fixed leading
parameter (`K : DINT`) with the variadic inputs (`args : {sized} U...`). All
pins of a variadic callee pass positionally in pin order, so the wired
selector lands in the `K` slot and the remaining inputs fill the variadic
list: `MUX(sel, a, b)`. The captured return is inferred from the variadic
candidates (`DINT`).

Illustrated:
```
          MUX (1)
        +---------------+
sel --> | K         MUX | --> result (2)
  a --> |               |
  b --> |               |
        +---------------+
        (pins 2 and 3 are unnamed in the export)
```
