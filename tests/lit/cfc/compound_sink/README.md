# compound_sink

End-to-end proof that a CFC sink may name a compound target. The sink writes
the function's result into `results[1]` — an array element, not a plain
identifier — so the transpiler must lower the target as a parsed expression.

```text
                    +----- function_0 -----+ (0)
   input  <-------->| inout     function_0 |--->  results[1]  (1)
                    +----------------------+

   <-->     an in-out pin (passed by reference)
   (0),(1)  evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the CFC program (same network as
  `compiler/plc_cfc/fixtures/valid/compound_sink`).
- `function_0.st` — the callee; increments its in-out and returns it.
- `main.st` — entry point; seeds `input` with 41 and prints `results[1]`,
  expecting 42.
