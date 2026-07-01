# pou_kinds

Integration test: **every CFC POU kind** — a `FUNCTION`, a `FUNCTION_BLOCK`, and a
`PROGRAM` — each defined in CFC and called from ST. All three share the same trivial
graph (add two inputs via the `MyAdd` wrapper, see `../OPEN.md`); only the POU kind and
the output sink differ. This also covers the "CFC called from ST" direction (the reverse
of `st_called_from_cfc`).

All three networks have the identical shape:

```text
                    +------ MyAdd ------+  (1)
   a  ------------->| in1         MyAdd |------------>  <out>  (2)
   b  ------------->| in2               |
                    +-------------------+

   <out>     CfcSum (the function result) | s (the FB output) | total (the program var)
   (1),(2)   evaluation-priority badges shown by the IDE
```

- `builtins.st` — the `MyAdd` wrapper.
- `cfc_sum.cfc` — `FUNCTION CfcSum : DINT` (result returned through a sink named after the function).
- `cfc_adder.cfc` — `FUNCTION_BLOCK CfcAdder` (result on the `s` output pin).
- `cfc_counter.cfc` — `PROGRAM CfcCounter` (result in the `total` program variable).
- `main.st` — entry point: calls `CfcSum(2, 3)`, an instance of `CfcAdder(4, 5)`, and the
  `CfcCounter` singleton (`6 + 7`), printing each result.

Proves all three CFC POU kinds lower, compile, link, run, and are callable from ST
(`func = 5`, `fb = 9`, `prog = 13`).
