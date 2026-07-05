# negated_input

A function call with a negated input pin. The negated `a` input wraps its
incoming value in `NOT` before passing it.

```text
                     +----- myGate -----+ (1)
   localA  --o------>| a         myGate |--->  localResult  (2)
   localB  --------->| b                |
                     +------------------+

   o        a negated input pin (wraps its value in NOT)
   (1),(2)  evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test (the FBD network above).
- `myGate.st` — the function it calls (`FUNCTION myGate : BOOL`, inputs `a`, `b`).

`localA` feeds the negated `a` pin and `localB` the plain `b` pin, so the network
means:

```text
temp_0 := myGate(a := NOT localA, b := localB);
localResult := temp_0;
```
