# operator_functions

What: The builtins named by an operator keyword: the variadic `AND`, `OR` and
`XOR`, the two-input `MOD` and the one-input `NOT`. The keyword is no ST
expression on its own, the block's `typeName` still becomes the callee:
`AND(a, b, c)`, `MOD(IN1 := x, IN2 := y)`, `NOT(IN := a)`.

Illustrated:
```
        AND (1)
      +-----------+
a --> | IN1   AND | --> and_result (6)
b --> |           |
c --> |           |
      +-----------+
      (pins 2 and 3 are unnamed in the export)

        OR (2)
      +-----------+
a --> | IN1    OR | --> or_result (7)
b --> |           |
      +-----------+
      (pin 2 is unnamed in the export)

        XOR (3)
      +-----------+
a --> | IN1   XOR | --> xor_result (8)
b --> |           |
      +-----------+
      (pin 2 is unnamed in the export)

        MOD (4)
      +-----------+
x --> | IN1   MOD | --> mod_result (9)
y --> | IN2       |
      +-----------+

        NOT (5)
      +-----------+
a --> | IN    NOT | --> not_result (10)
      +-----------+
```
