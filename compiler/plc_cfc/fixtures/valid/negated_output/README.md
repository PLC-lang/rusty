# negated_output

A function call with a negated output pin. The negated `myGate` output pin
inverts the value it feeds downstream, so every consumer reads `NOT` the
block's result.

```text
                     +----- myGate -----+ (1)
   localA  --------->| a         myGate |--o--->  localResult  (2)
   localB  --------->| b                |
                     +------------------+

   o        a negated output pin (consumers read NOT the pin's value)
   (1),(2)  evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test (the FBD network above).
- `myGate.st` — the function it calls (`FUNCTION myGate : BOOL`, inputs `a`, `b`).

The call itself is unchanged; the negation is applied where the pin's value is
consumed, so the network means:

```text
__myGate_res_0 := myGate(a := localA, b := localB);
localResult := NOT __myGate_res_0;
```
