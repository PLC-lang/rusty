# generic_call

End-to-end proof that a CFC block may call a generic builtin. `SEL` is declared
`FUNCTION SEL<U: ANY> : U`, so the result temporary's type cannot be read off
the declaration (that is the generic marker `__SEL__U`) — it is resolved from
the annotated call, where the annotator specialized `U` to `DINT`.

```text
   TRUE      --------->+------ SEL ------+ (2)
   localIn0  --------->| G           SEL |--->  result  (3)
   localIn1  --------->| IN0             |
                       | IN1             |
                       +-----------------+

   (2),(3)  evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the CFC program (an IDE export); `G` is wired to the
  literal `TRUE`, so `SEL` selects `IN1`.
- `main.st` — entry point; seeds `localIn0 = 10`, `localIn1 = 20` and prints
  `result`, expecting 20.
