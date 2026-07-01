# inout

Integration test: a **VAR_IN_OUT (by-reference) pin**. The CFC program `Accumulator`
calls the ST function `accumulate`, binding the in-out parameter `sum` to its own `sum`
variable. The callee reads *and writes* `sum` in place, so the caller's variable reflects
the mutation after the call — not just the returned value.

```text
                    +------ accumulate ------+ (1)
   value  --------->| value                  |
                    |              accumulate |--->  result  (2)
   sum  <---------->| sum                     |
                    +-------------------------+

   <-->     an in-out pin (passed by reference)
   (1),(2)  evaluation-priority badges shown by the IDE
```

- `accumulate.st` — `FUNCTION accumulate` (input `value`, in-out `sum`): `sum := sum + value; accumulate := sum;`.
- `accumulator.cfc` — `PROGRAM Accumulator` wiring `value`/`sum` into the call, `accumulate`'s result to `result`.
- `main.st` — entry point: seeds `sum := 5`, `value := 10`, runs once, prints `sum` and `result`.

Lowers to:

```text
__temp_0 := accumulate(value := value, sum := sum);
result := __temp_0;
```

`sum` is passed by reference, so after the call `sum` is `15` (5 + 10), proving the
by-reference mutation runs — `result` is `15` too.
