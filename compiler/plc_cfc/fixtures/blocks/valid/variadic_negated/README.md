What: negation bubbles on the extensible pins of the builtin variadic `ADD`:
one wire negated on the source side, one on the pin side, one on both (the
NOTs nest). Extended variadic pins export with an empty `parameterName`; the
values pass by pin order. Verbatim IDE export.

Illustrated:
```
          ADD (1)
        +---------------+
a1 o--> | IN1       ADD | --> b1 (2)
a2 ---o |               |
a3 o--o |               |
        +---------------+
        (o = negated pin)
```
