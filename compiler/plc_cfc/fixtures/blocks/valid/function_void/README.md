What: A void function `myVoid` (no return value, hence no return pin) called
with input `in` from `localIn` (0); its `out` pin is read into `localOut` (1).
Structurally the block looks exactly like a program call — no `instanceName`,
no return pin — so classifying it as stateless needs the callee's indexed kind.
Its consumed output is captured into a typed temporary, not read as `myVoid.out`.

Illustrated:
```
localIn --> in [myVoid] out (0) --> localOut (1)
```
