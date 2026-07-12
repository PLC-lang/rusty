# negated_output_fan_out

A function block with a negated output pin feeding **two** consumers. The
negation belongs to the pin, not to a single wire, so *every* consumer reads
`NOT` the pin's value. One of the consumers is itself a negated input pin,
which composes with the output negation to a double `NOT`.

```text
                       +----- Toggle -----+ (1)
   localEnable  ------>| enable      isOn |--o--+---->  localOff  (2)
                       +------------------+     |
                                                |    +----- myGate -----+ (3)
                                                +--o-| a         myGate |--->  localResult  (4)
                       localB  --------------------->| b                |
                                                     +------------------+

   o (pin)   the negated isOn output pin (consumers read NOT the pin's value)
   o (wire)  the negated a input pin (wraps its value in NOT)
   (1)-(4)   evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test (the FBD network above).
- `Toggle.st` — the function block it calls (`FUNCTION_BLOCK Toggle`, input
  `enable`, output `isOn`).
- `myGate.st` — the function it calls (`FUNCTION myGate : BOOL`, inputs `a`, `b`).

Unlike `negated_output` (a function's return pin), `isOn` is an ordinary output
pin, so it is routed through an output argument (`isOn => <temp>`) and the
negation is applied at each of the two consumption sites:

```text
myInstance(enable := localEnable, isOn => __Toggle_res_0);
localOff := NOT __Toggle_res_0;
__myGate_res_1 := myGate(a := NOT NOT __Toggle_res_0, b := localB);
localResult := __myGate_res_1;
```
