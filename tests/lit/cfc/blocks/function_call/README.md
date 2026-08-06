What: the CFC POU is itself a `FUNCTION` with inputs, a return value, and a
declared output, its body one `myAdd` call wiring the POU's interface through
the block. `main` calls it directly (`function_call(in1 := 3, in2 := 4,
doubledOut => doubled)`) and checks `result = 7, doubled = 14`.

Illustrated:
```
in1 --> in1 [myAdd] myAdd        --> function_call (0, 1)
in2 --> in2         myAddDoubled --> doubledOut    (2)
```
