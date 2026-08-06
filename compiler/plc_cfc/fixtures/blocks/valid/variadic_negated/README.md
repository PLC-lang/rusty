What: negation bubbles on the extensible pins of the builtin variadic `ADD`:
one wire negated on the source side, one on the pin side, one on both (the
NOTs nest). Extended variadic pins export with an empty `parameterName`; the
values pass by pin order. Verbatim IDE export.

Illustrated:
```
           +------ ADD ------+
a1 o-------| IN1             |
a2 ------o-|             ADD |--------> b1 (1)
a3 o-----o-|                 |
           +-----------------+ (0)
```
