What: a conditional early return in a program that has a `VAR_TEMP` instance of
a function block with `FB_EXIT`. `main` checks that the temporary is exited one
time for each call: when the guard is false and the body runs to its end
(`result = 42`), and when the guard is true and the call returns early
(`result = 7`).

Illustrated:
```
guard --> RETURN (1)
42 --> result (2)
```
