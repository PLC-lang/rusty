What: The function `myAdd` called with `in1`/`in2`; its return pin (the unnamed
output pin, drawn as `myAdd` below) feeds the enclosing function's result, and
its output pin `myAddDoubled` feeds `doubledOut`. A stateless callee: both reads go
through generated persistent temporaries captured by the call, not `inst.member`.

Illustrated:
```
          myAdd (0)
        +--------------------+
in1 --> | in1          myAdd | --> function_call (1)
in2 --> | in2   myAddDoubled | --> doubledOut (2)
        +--------------------+
```
