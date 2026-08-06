What: the full block negation matrix at runtime (the verbatim `function_args`
IDE export): a wire negated on both ends (`in1 := NOT NOT a1`, and the
return-pin-to-`b1` wire), and wires negated on the variable side only
(`in2 := NOT a2`, `b2 := NOT myAddDoubled`). NOT on DINT is the bitwise
complement, so `main` checks `b1 = 6, b2 = -13` for `a1 = 10, a2 = 3`.

Illustrated:
```
a1 o---o in1 [myAdd] myAdd        o---o b1 (0, 1)
a2 o---- in2 [     ] myAddDoubled ----o b2 (2)
```
