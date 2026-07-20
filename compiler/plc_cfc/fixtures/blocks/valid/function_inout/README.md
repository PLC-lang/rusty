What: The function `addInto` with a `VAR_IN_OUT acc` wired to the variable
`total`; `delta` takes the literal `5` and the return feeds `result`. The in_out
is passed by reference (`acc := total`), so `total` is mutated in place across
cycles — the persistent-variable analogue of a supplied in_out.

Illustrated:
```
          addInto (0)
        +--------------------+
  5 --> | delta      addInto | --> result (1)
total <=> acc                |
        +--------------------+
```
