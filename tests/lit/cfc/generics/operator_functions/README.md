What: calls to the builtins named by an operator keyword: the variadic `AND`,
`OR` and `XOR`, the two-input `MOD` and the one-input `NOT`. `main` runs the
diagram twice and checks each result against its ST operator.

Illustrated:
```
        AND (1)
      +-----------+
a --> | IN1   AND | --> and_result (6)
b --> |           |
c --> |           |
      +-----------+

        OR (2)
      +-----------+
a --> | IN1    OR | --> or_result (7)
b --> |           |
      +-----------+

        XOR (3)
      +-----------+
a --> | IN1   XOR | --> xor_result (8)
b --> |           |
      +-----------+

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
