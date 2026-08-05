# variadic_add

What: The builtin variadic `ADD` with three inputs — a variable, another
variable, and a literal. Variadic pins carry no usable parameter names (the
IDE exports the first as `IN1` and the rest empty, see `reference/add2.cfc`),
so the call passes its arguments positionally in pin order. The captured
return is inferred as the promoted candidate type (`DINT`).

Illustrated:
```
        ADD (0)
      +---------------+
x --> | IN1       ADD | --> result (1)
y --> |               |
5 --> |               |
      +---------------+
      (pins 2 and 3 are unnamed in the export)
```
