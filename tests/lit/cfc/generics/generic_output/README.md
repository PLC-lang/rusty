What: a generic function without a return value whose generic type is decided
by its input, read through the declared output pin. `main` checks the doubled
value (`y = 42` for `x = 21`).

Illustrated:
```
        myGenOut (1)
      +-------------------+
x --> | a         doubled | --> y (2)
      +-------------------+
```
