# generic_output

What: A generic function whose *output parameter* is generic (`myGenOut<T:
ANY_NUM>` with `VAR_OUTPUT doubled : T`, no return value). The consumed output
pin is captured into a temporary via `doubled => __out_doubled_1` inside a
*bare* call — there is no return-capture assignment — so inference must
harvest the type from the annotated output parameter, not the call itself.
`x : DINT` decides `T = DINT`.

Illustrated:
```
        myGenOut (0)
      +------------------+
x --> | a        doubled | --> y (1)
      +------------------+
```
