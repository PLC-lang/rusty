# st_called_from_cfc

Integration test: a **CFC function block calls an ST function**. `Computer` (a CFC FB)
wires its two inputs into a `MyAdd` block, whose output becomes the FB's `result`
output. `MyAdd` is an ordinary ST function (a stand-in for an `ADD` operator block —
see `../OPEN.md`). An ST `main` drives it and prints the result.

```text
                    +------ MyAdd ------+  (1)
   a  ------------->| in1         MyAdd |------------>  result  (2)
   b  ------------->| in2               |
                    +-------------------+

   MyAdd     an ST wrapper function (builtins.st), standing in for an ADD block
   (1),(2)   evaluation-priority badges shown by the IDE
```

- `builtins.st` — the `MyAdd` wrapper (`MyAdd := in1 + in2;`).
- `computer.cfc` — the CFC function block under test (`a, b : DINT` in, `result : DINT` out).
- `main.st` — entry point: instantiates `Computer`, calls it with `20, 22`, prints `result`.

`MyAdd`'s result feeds the `result` sink, so it is evaluated once into a temporary and
then assigned — the FB body lowers to:

```text
__temp_0 := MyAdd(in1 := a, in2 := b);
result := __temp_0;
```

This proves a CFC→ST call compiles, links, and runs (`result = 42`), and incidentally
that the `__return@MyAdd` temp placeholder resolves to `DINT` against the real index.
